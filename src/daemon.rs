use std::fmt;
use tokio::sync::{broadcast, mpsc};
use std::sync::Arc;
use zbus::{Connection, proxy};

use crate::error::HachiError;

/// Power profile modes for ASUS laptops
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PowerProfile {
    Quiet,
    #[default]
    Balanced,
    Performance,
}

impl PowerProfile {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Quiet => "Quiet",
            Self::Balanced => "Balanced",
            Self::Performance => "Performance",
        }
    }

    pub fn from_u8(val: u8) -> Self {
        Self::from_u32(val as u32)
    }

    pub fn from_u32(val: u32) -> Self {
        // asusd 6.x: 0=Balanced, 1=Performance, 3=Quiet(LowPower)
        match val {
            0 => Self::Balanced,
            1 => Self::Performance,
            3 => Self::Quiet,
            _ => Self::Balanced,
        }
    }

    pub fn to_u8(self) -> u8 {
        self.to_u32() as u8
    }

    pub fn to_u32(self) -> u32 {
        // asusd 6.x: 0=Balanced, 1=Performance, 3=Quiet(LowPower)
        match self {
            Self::Balanced => 0,
            Self::Performance => 1,
            Self::Quiet => 3,
        }
    }

    pub fn cycle_next(self) -> Self {
        match self {
            Self::Quiet => Self::Balanced,
            Self::Balanced => Self::Performance,
            Self::Performance => Self::Quiet,
        }
    }
}

impl fmt::Display for PowerProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Fan curve point (temperature in °C, fan speed in %)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FanPoint {
    pub temp: u8,
    pub speed: u8,
}

/// Fan curve data
#[derive(Debug, Clone, Default)]
pub struct FanCurve {
    pub cpu_curve: Vec<FanPoint>,
    pub gpu_curve: Vec<FanPoint>,
    pub enabled: bool,
}

impl FanCurve {
    pub fn default_curve() -> Self {
        Self {
            cpu_curve: vec![
                FanPoint { temp: 30, speed: 0 },
                FanPoint { temp: 40, speed: 5 },
                FanPoint { temp: 50, speed: 10 },
                FanPoint { temp: 60, speed: 20 },
                FanPoint { temp: 70, speed: 35 },
                FanPoint { temp: 80, speed: 55 },
                FanPoint { temp: 90, speed: 65 },
                FanPoint { temp: 100, speed: 100 },
            ],
            gpu_curve: vec![
                FanPoint { temp: 30, speed: 0 },
                FanPoint { temp: 40, speed: 5 },
                FanPoint { temp: 50, speed: 10 },
                FanPoint { temp: 60, speed: 20 },
                FanPoint { temp: 70, speed: 35 },
                FanPoint { temp: 80, speed: 55 },
                FanPoint { temp: 90, speed: 65 },
                FanPoint { temp: 100, speed: 100 },
            ],
            enabled: false,
        }
    }
}

/// Current hardware state snapshot
#[derive(Debug, Clone, Default)]
pub struct HardwareState {
    pub power_profile: PowerProfile,
    pub charge_limit: u8,
    pub fan_curve: FanCurve,
    pub connected: bool,
}

/// Intents sent from UI to Hardware Actor
#[derive(Debug, Clone)]
pub enum HardwareIntent {
    /// Request current state refresh
    RefreshState,
    /// Set power profile
    SetPowerProfile(PowerProfile),
    /// Set battery charge limit (0-100)
    SetChargeLimit(u8),
    /// Set custom fan curve
    SetFanCurve(FanCurve),
    /// Enable/disable custom fan curves
    SetFanCurveEnabled(bool),
    /// Shutdown the actor
    Shutdown,
}

/// Updates broadcast from Hardware Actor to UI
#[derive(Debug, Clone)]
pub enum HardwareUpdate {
    /// Full state refresh
    StateRefresh(HardwareState),
    /// Power profile changed
    PowerProfileChanged(PowerProfile),
    /// Charge limit changed
    ChargeLimitChanged(u8),
    /// Fan curve changed
    FanCurveChanged(FanCurve),
    /// Connection status changed
    ConnectionStatus(bool),
    /// Error occurred
    Error(Arc<HachiError>),
}

// =============================================================================
// D-Bus Proxy Definitions for org.asuslinux.Daemon
// =============================================================================

#[proxy(
    interface = "xyz.ljones.Platform",
    default_service = "xyz.ljones.Asusd",
    default_path = "/xyz/ljones"
)]
trait AsusPlatform {
    /// Get the current platform profile (0=Quiet, 1=Balanced, 2=Performance, 3=?)
    #[zbus(property)]
    fn platform_profile(&self) -> zbus::Result<u32>;

    /// Set the platform profile
    #[zbus(property)]
    fn set_platform_profile(&self, profile: u32) -> zbus::Result<()>;

    /// Get charge control end threshold (battery limit)
    #[zbus(property)]
    fn charge_control_end_threshold(&self) -> zbus::Result<u8>;

    /// Set charge control end threshold
    #[zbus(property)]
    fn set_charge_control_end_threshold(&self, limit: u8) -> zbus::Result<()>;

    /// Cycle to next platform profile
    fn next_platform_profile(&self) -> zbus::Result<()>;
}

// =============================================================================
// Hardware Actor Implementation
// =============================================================================

pub struct HardwareActor {
    intent_rx: mpsc::Receiver<HardwareIntent>,
    update_tx: broadcast::Sender<HardwareUpdate>,
    connection: Option<Connection>,
}

impl HardwareActor {
    pub fn new(
        intent_rx: mpsc::Receiver<HardwareIntent>,
        update_tx: broadcast::Sender<HardwareUpdate>,
    ) -> Self {
        Self {
            intent_rx,
            update_tx,
            connection: None,
        }
    }

    /// Run the actor loop
    pub async fn run(mut self) {
        use futures::StreamExt;

        // Try to establish D-Bus connection
        self.connect().await;

        // Initial state fetch
        if self.connection.is_some() {
            self.refresh_state().await;
        }

        // Set up property change monitoring
        let mut property_stream = if let Some(conn) = &self.connection {
            match AsusPlatformProxy::new(conn).await {
                Ok(proxy) => Some(proxy.receive_platform_profile_changed().await),
                Err(_) => None,
            }
        } else {
            None
        };

        // Main event loop using select
        loop {
            tokio::select! {
                // Handle intents from UI
                Some(intent) = self.intent_rx.recv() => {
                    match intent {
                        HardwareIntent::RefreshState => {
                            self.refresh_state().await;
                        }
                        HardwareIntent::SetPowerProfile(profile) => {
                            self.set_power_profile(profile).await;
                        }
                        HardwareIntent::SetChargeLimit(limit) => {
                            self.set_charge_limit(limit).await;
                        }
                        HardwareIntent::SetFanCurve(curve) => {
                            self.set_fan_curve(curve).await;
                        }
                        HardwareIntent::SetFanCurveEnabled(enabled) => {
                            self.set_fan_curve_enabled(enabled).await;
                        }
                        HardwareIntent::Shutdown => {
                            break;
                        }
                    }
                }

                // Handle property changes from D-Bus
                Some(change) = async {
                    match &mut property_stream {
                        Some(stream) => stream.next().await,
                        None => std::future::pending().await,
                    }
                } => {
                    if let Ok(profile) = change.get().await {
                        let new_profile = PowerProfile::from_u32(profile);
                        let _ = self.update_tx.send(HardwareUpdate::PowerProfileChanged(new_profile));
                    }
                }

                else => break,
            }
        }
    }

    async fn connect(&mut self) {
        match Connection::system().await {
            Ok(conn) => {
                self.connection = Some(conn);
                let _ = self.update_tx.send(HardwareUpdate::ConnectionStatus(true));
            }
            Err(e) => {
                let _ = self.update_tx.send(HardwareUpdate::Error(Arc::new(
                    HachiError::from(e)
                )));
                let _ = self.update_tx.send(HardwareUpdate::ConnectionStatus(false));
            }
        }
    }

    async fn refresh_state(&mut self) {
        let Some(conn) = &self.connection else {
            return;
        };

        let mut state = HardwareState {
            connected: true,
            ..Default::default()
        };

        // Fetch power profile and charge limit from Platform interface
        if let Ok(proxy) = AsusPlatformProxy::new(conn).await {
            if let Ok(profile) = proxy.platform_profile().await {
                state.power_profile = PowerProfile::from_u32(profile);
            }
            if let Ok(limit) = proxy.charge_control_end_threshold().await {
                state.charge_limit = limit;
            }
        }

        // Use default fan curve (fan curves interface may not be available)
        state.fan_curve = FanCurve::default_curve();

        let _ = self.update_tx.send(HardwareUpdate::StateRefresh(state));
    }

    async fn set_power_profile(&mut self, profile: PowerProfile) {
        let Some(conn) = &self.connection else {
            let _ = self.update_tx.send(HardwareUpdate::Error(Arc::new(
                HachiError::DbusCall("Not connected to D-Bus".to_string())
            )));
            return;
        };

        match AsusPlatformProxy::new(conn).await {
            Ok(proxy) => {
                if let Err(e) = proxy.set_platform_profile(profile.to_u32()).await {
                    let _ = self.update_tx.send(HardwareUpdate::Error(Arc::new(
                        HachiError::from(e)
                    )));
                } else {
                    let _ = self
                        .update_tx
                        .send(HardwareUpdate::PowerProfileChanged(profile));
                }
            }
            Err(e) => {
                let _ = self.update_tx.send(HardwareUpdate::Error(Arc::new(
                    HachiError::from(e)
                )));
            }
        }
    }

    async fn set_charge_limit(&mut self, limit: u8) {
        let Some(conn) = &self.connection else {
            let _ = self.update_tx.send(HardwareUpdate::Error(Arc::new(
                HachiError::DbusCall("Not connected to D-Bus".to_string())
            )));
            return;
        };

        let limit = limit.clamp(20, 100);

        match AsusPlatformProxy::new(conn).await {
            Ok(proxy) => {
                if let Err(e) = proxy.set_charge_control_end_threshold(limit).await {
                    let _ = self.update_tx.send(HardwareUpdate::Error(Arc::new(
                        HachiError::from(e)
                    )));
                } else {
                    let _ = self
                        .update_tx
                        .send(HardwareUpdate::ChargeLimitChanged(limit));
                }
            }
            Err(e) => {
                let _ = self.update_tx.send(HardwareUpdate::Error(Arc::new(
                    HachiError::from(e)
                )));
            }
        }
    }

    async fn set_fan_curve(&mut self, curve: FanCurve) {
        // Fan curves not yet supported in this asusd version
        let _ = self.update_tx.send(HardwareUpdate::Error(Arc::new(
            HachiError::InvalidFanCurve("Fan curve control not available".to_string())
        )));
        // Still update local state for UI feedback
        let _ = self.update_tx.send(HardwareUpdate::FanCurveChanged(curve));
    }

    async fn set_fan_curve_enabled(&mut self, _enabled: bool) {
        // Fan curves not yet supported in this asusd version
        let _ = self.update_tx.send(HardwareUpdate::Error(Arc::new(
            HachiError::InvalidFanCurve("Fan curve control not available".to_string())
        )));
    }
}

// =============================================================================
// Actor Handle (for UI thread to communicate with actor)
// =============================================================================

pub struct DaemonHandle {
    intent_tx: mpsc::Sender<HardwareIntent>,
    update_rx: broadcast::Receiver<HardwareUpdate>,
}

impl DaemonHandle {
    /// Spawn the hardware actor and return a handle
    pub fn spawn() -> Self {
        let (intent_tx, intent_rx) = mpsc::channel(32);
        let (update_tx, update_rx) = broadcast::channel(64);

        let actor = HardwareActor::new(intent_rx, update_tx);

        tokio::spawn(async move {
            actor.run().await;
        });

        Self {
            intent_tx,
            update_rx,
        }
    }

    /// Send an intent to the hardware actor (non-blocking)
    pub fn send(&self, intent: HardwareIntent) {
        let _ = self.intent_tx.try_send(intent);
    }

    /// Request a state refresh
    pub fn refresh(&self) {
        self.send(HardwareIntent::RefreshState);
    }

    /// Set power profile
    pub fn set_power_profile(&self, profile: PowerProfile) {
        self.send(HardwareIntent::SetPowerProfile(profile));
    }

    /// Set battery charge limit
    pub fn set_charge_limit(&self, limit: u8) {
        self.send(HardwareIntent::SetChargeLimit(limit));
    }

    /// Set fan curve
    pub fn set_fan_curve(&self, curve: FanCurve) {
        self.send(HardwareIntent::SetFanCurve(curve));
    }

    /// Toggle fan curve control
    pub fn set_fan_curve_enabled(&self, enabled: bool) {
        self.send(HardwareIntent::SetFanCurveEnabled(enabled));
    }

    /// Try to receive an update (non-blocking)
    pub fn try_recv(&mut self) -> Option<HardwareUpdate> {
        self.update_rx.try_recv().ok()
    }

    /// Shutdown the actor
    pub fn shutdown(&self) {
        let _ = self.intent_tx.try_send(HardwareIntent::Shutdown);
    }
}
