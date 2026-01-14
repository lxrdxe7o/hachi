use std::time::{Duration, Instant};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use crate::daemon::{DaemonHandle, HardwareState, HardwareUpdate, PowerProfile};
use crate::ui::{
    colors, BatteryKatana, EffectManager, FanCurveGraph, Header, HelpPopup,
    PowerProfileSelector, SakuraShader, StatusBar,
};

/// Which panel is currently focused
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusedPanel {
    PowerProfile,
    Battery,
    FanCurve,
}

impl FocusedPanel {
    pub fn next(self) -> Self {
        match self {
            Self::PowerProfile => Self::Battery,
            Self::Battery => Self::FanCurve,
            Self::FanCurve => Self::PowerProfile,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::PowerProfile => Self::FanCurve,
            Self::Battery => Self::PowerProfile,
            Self::FanCurve => Self::Battery,
        }
    }
}

/// Edit mode for interactive widgets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditMode {
    None,
    Battery,
    FanCurve { point_index: usize },
}

/// Main application state
pub struct App {
    /// Hardware actor handle
    daemon: DaemonHandle,

    /// Shadow state (local copy for immediate UI feedback)
    pub state: HardwareState,

    /// Currently focused panel
    pub focused: FocusedPanel,

    /// Currently selected power profile index
    pub selected_profile: usize,

    /// Current edit mode
    pub edit_mode: EditMode,

    /// Whether help popup is visible
    pub show_help: bool,

    /// Status message to display
    pub status_message: Option<(String, Instant)>,

    /// Effect manager for TachyonFX
    pub effects: EffectManager,

    /// Sakura particle shader
    pub sakura: Option<SakuraShader>,

    /// Whether sakura particles are visible
    pub sakura_enabled: bool,

    /// Whether app should quit
    pub should_quit: bool,

    /// Last frame time for delta calculations
    last_frame: Instant,
}

impl App {
    pub fn new(daemon: DaemonHandle) -> Self {
        Self {
            daemon,
            state: HardwareState::default(),
            focused: FocusedPanel::PowerProfile,
            selected_profile: 1, // Balanced by default
            edit_mode: EditMode::None,
            show_help: false,
            status_message: None,
            effects: EffectManager::new(),
            sakura: None,
            sakura_enabled: true,
            should_quit: false,
            last_frame: Instant::now(),
        }
    }

    /// Initialize sakura shader with terminal dimensions
    pub fn init_sakura(&mut self, width: u16, height: u16) {
        let density = ((width as usize * height as usize) / 80).clamp(10, 100);
        self.sakura = Some(SakuraShader::new(width, height, density));
    }

    /// Process any pending hardware updates
    pub fn process_updates(&mut self) {
        while let Some(update) = self.daemon.try_recv() {
            match update {
                HardwareUpdate::StateRefresh(new_state) => {
                    self.state = new_state;
                    // Map PowerProfile to UI index: Quiet=0, Balanced=1, Performance=2
                    self.selected_profile = match self.state.power_profile {
                        PowerProfile::Quiet => 0,
                        PowerProfile::Balanced => 1,
                        PowerProfile::Performance => 2,
                    };
                }
                HardwareUpdate::PowerProfileChanged(profile) => {
                    self.state.power_profile = profile;
                    // Sync UI selection with new profile
                    self.selected_profile = match profile {
                        PowerProfile::Quiet => 0,
                        PowerProfile::Balanced => 1,
                        PowerProfile::Performance => 2,
                    };
                    self.set_status(format!("Profile changed to {}", profile));
                }
                HardwareUpdate::ChargeLimitChanged(limit) => {
                    self.state.charge_limit = limit;
                    self.set_status(format!("Charge limit set to {}%", limit));
                }
                HardwareUpdate::FanCurveChanged(curve) => {
                    self.state.fan_curve = curve;
                    self.set_status("Fan curve updated".to_string());
                }
                HardwareUpdate::ConnectionStatus(connected) => {
                    self.state.connected = connected;
                    if !connected {
                        self.set_status("Disconnected from daemon".to_string());
                    }
                }
                HardwareUpdate::Error(msg) => {
                    self.set_status(format!("Error: {}", msg));
                }
            }
        }

        // Clear old status messages (after 5 seconds)
        if let Some((_, time)) = &self.status_message {
            if time.elapsed() > Duration::from_secs(5) {
                self.status_message = None;
            }
        }
    }

    /// Set a status message
    fn set_status(&mut self, msg: String) {
        self.status_message = Some((msg, Instant::now()));
    }

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        // Global keys
        match key.code {
            KeyCode::Char('q') if self.edit_mode == EditMode::None => {
                self.should_quit = true;
                return;
            }
            KeyCode::Char('?') if self.edit_mode == EditMode::None => {
                self.show_help = !self.show_help;
                return;
            }
            KeyCode::Esc => {
                if self.show_help {
                    self.show_help = false;
                } else if self.edit_mode != EditMode::None {
                    self.edit_mode = EditMode::None;
                }
                return;
            }
            KeyCode::Char('r') if self.edit_mode == EditMode::None => {
                self.daemon.refresh();
                self.set_status("Refreshing state...".to_string());
                return;
            }
            KeyCode::Char('s') if self.edit_mode == EditMode::None => {
                self.sakura_enabled = !self.sakura_enabled;
                let status = if self.sakura_enabled { "Sakura enabled" } else { "Sakura disabled" };
                self.set_status(status.to_string());
                return;
            }
            _ => {}
        }

        // Don't process other keys if help is showing
        if self.show_help {
            return;
        }

        // Handle edit mode input
        match self.edit_mode {
            EditMode::Battery => self.handle_battery_edit(key),
            EditMode::FanCurve { point_index } => self.handle_fan_curve_edit(key, point_index),
            EditMode::None => self.handle_navigation(key),
        }
    }

    /// Handle navigation when not in edit mode
    fn handle_navigation(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Tab | KeyCode::Char('L') => {
                self.focused = self.focused.next();
            }
            KeyCode::BackTab | KeyCode::Char('H') => {
                self.focused = self.focused.prev();
            }
            KeyCode::Char('1') => {
                self.focused = FocusedPanel::PowerProfile;
            }
            KeyCode::Char('2') => {
                self.focused = FocusedPanel::Battery;
            }
            KeyCode::Char('3') => {
                self.focused = FocusedPanel::FanCurve;
            }
            KeyCode::Up | KeyCode::Char('k') => match self.focused {
                FocusedPanel::PowerProfile => {
                    self.selected_profile = self.selected_profile.saturating_sub(1);
                }
                _ => {}
            },
            KeyCode::Down | KeyCode::Char('j') => match self.focused {
                FocusedPanel::PowerProfile => {
                    self.selected_profile = (self.selected_profile + 1).min(2);
                }
                _ => {}
            },
            KeyCode::Enter | KeyCode::Char(' ') => match self.focused {
                FocusedPanel::PowerProfile => {
                    // UI index: 0=Quiet, 1=Balanced, 2=Performance
                    let new_profile = match self.selected_profile {
                        0 => PowerProfile::Quiet,
                        1 => PowerProfile::Balanced,
                        2 => PowerProfile::Performance,
                        _ => PowerProfile::Balanced,
                    };
                    if new_profile != self.state.power_profile {
                        self.daemon.set_power_profile(new_profile);
                        // Optimistic update for immediate feedback
                        self.state.power_profile = new_profile;
                    }
                }
                FocusedPanel::Battery => {
                    self.edit_mode = EditMode::Battery;
                }
                FocusedPanel::FanCurve => {
                    self.edit_mode = EditMode::FanCurve { point_index: 0 };
                }
            },
            _ => {}
        }
    }

    /// Handle battery edit mode input
    fn handle_battery_edit(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Left | KeyCode::Char('h') => {
                self.state.charge_limit = self.state.charge_limit.saturating_sub(5).max(20);
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.state.charge_limit = (self.state.charge_limit + 5).min(100);
            }
            KeyCode::Enter => {
                self.daemon.set_charge_limit(self.state.charge_limit);
                self.edit_mode = EditMode::None;
            }
            _ => {}
        }
    }

    /// Handle fan curve edit mode input
    fn handle_fan_curve_edit(&mut self, key: crossterm::event::KeyEvent, point_index: usize) {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Left | KeyCode::Char('h') => {
                if point_index > 0 {
                    self.edit_mode = EditMode::FanCurve {
                        point_index: point_index - 1,
                    };
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if point_index < self.state.fan_curve.cpu_curve.len() - 1 {
                    self.edit_mode = EditMode::FanCurve {
                        point_index: point_index + 1,
                    };
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if let Some(point) = self.state.fan_curve.cpu_curve.get_mut(point_index) {
                    point.speed = (point.speed + 5).min(100);
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if let Some(point) = self.state.fan_curve.cpu_curve.get_mut(point_index) {
                    point.speed = point.speed.saturating_sub(5);
                }
            }
            KeyCode::Enter => {
                self.daemon.set_fan_curve(self.state.fan_curve.clone());
                self.edit_mode = EditMode::None;
            }
            _ => {}
        }
    }

    /// Update frame timing and effects
    pub fn tick(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame);
        self.last_frame = now;

        // Update sakura particles
        if let Some(ref mut sakura) = self.sakura {
            sakura.update(delta);
        }
    }

    /// Render the application
    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();

        // Clear with void black background
        let buf = frame.buffer_mut();
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_char(' ')
                        .set_bg(colors::VOID_BLACK)
                        .set_fg(colors::GHOST_WHITE);
                }
            }
        }

        // Render sakura particles in background (if enabled)
        if self.sakura_enabled {
            if let Some(ref sakura) = self.sakura {
                sakura.render(buf, area);
            }
        }

        // Main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(12),  // Header (compact)
                Constraint::Min(10),    // Main content
                Constraint::Length(1),  // Status bar
            ])
            .split(area);

        // Render header
        Header::new().render(chunks[0], buf);

        // Main content area
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(35), // Left panel
                Constraint::Percentage(65), // Right panel
            ])
            .margin(1)
            .split(chunks[1]);

        // Left panel: Power Profile + Battery
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10), // Power profile
                Constraint::Min(6),     // Battery
            ])
            .split(content_chunks[0]);

        // Render power profile selector
        PowerProfileSelector::new(self.state.power_profile)
            .selected(self.selected_profile)
            .focused(self.focused == FocusedPanel::PowerProfile)
            .render(left_chunks[0], buf);

        // Render battery katana
        BatteryKatana::new(self.state.charge_limit)
            .focused(self.focused == FocusedPanel::Battery)
            .editing(self.edit_mode == EditMode::Battery)
            .render(left_chunks[1], buf);

        // Right panel: Fan curve
        let fan_selected_point = match self.edit_mode {
            EditMode::FanCurve { point_index } => Some(point_index),
            _ => None,
        };

        FanCurveGraph::new(&self.state.fan_curve)
            .selected_point(fan_selected_point)
            .focused(self.focused == FocusedPanel::FanCurve)
            .editing(matches!(self.edit_mode, EditMode::FanCurve { .. }))
            .render(content_chunks[1], buf);

        // Render status bar
        let mut status_bar = StatusBar::new(self.state.connected);
        if let Some((ref msg, _)) = self.status_message {
            status_bar = status_bar.message(msg);
        }
        status_bar.render(chunks[2], buf);

        // Render help popup if visible
        if self.show_help {
            let popup_area = centered_rect(50, 60, area);
            HelpPopup.render(popup_area, buf);
        }

        // Process effects
        let delta = Duration::from_millis(16); // ~60fps
        self.effects.process(delta, buf, area);
    }

    /// Handle terminal resize
    pub fn resize(&mut self, width: u16, height: u16) {
        if let Some(ref mut sakura) = self.sakura {
            sakura.resize(width, height);
        }
    }

    /// Shutdown the daemon actor
    pub fn shutdown(&self) {
        self.daemon.shutdown();
    }
}

/// Helper to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
