use ratatui::style::{Color, Modifier, Style};

/// Ronin Cyberpunk color palette
pub mod colors {
    use ratatui::style::Color;

    /// Void Black - Deep background
    pub const VOID_BLACK: Color = Color::Rgb(13, 13, 21);

    /// Neon Cyan - Active elements, highlights (Muted)
    pub const NEON_CYAN: Color = Color::Rgb(60, 203, 225);

    /// Sakura Pink - Particles, secondary highlights
    pub const SAKURA_PINK: Color = Color::Rgb(255, 0, 85);

    /// Ronin Red - Critical, errors, warnings (Muted)
    pub const RONIN_RED: Color = Color::Rgb(225, 60, 60);

    /// Ghost White - Primary text
    pub const GHOST_WHITE: Color = Color::Rgb(230, 230, 240);

    /// Steel Gray - Secondary text, borders (Softer)
    pub const STEEL_GRAY: Color = Color::Rgb(100, 100, 120);

    /// Shadow Gray - Subtle backgrounds
    pub const SHADOW_GRAY: Color = Color::Rgb(25, 25, 35);

    /// Ember Orange - Performance mode accent (Muted)
    pub const EMBER_ORANGE: Color = Color::Rgb(225, 130, 40);

    /// Zen Purple - Quiet mode accent
    pub const ZEN_PURPLE: Color = Color::Rgb(138, 43, 226);

    /// Balance Blue - Balanced mode accent
    pub const BALANCE_BLUE: Color = Color::Rgb(0, 150, 255);
}

/// Pre-defined styles for UI consistency
pub mod styles {
    use super::colors::*;
    use ratatui::style::{Modifier, Style};

    /// Default text style
    pub fn text() -> Style {
        Style::default().fg(GHOST_WHITE)
    }

    /// Dimmed/secondary text
    pub fn text_dim() -> Style {
        Style::default().fg(STEEL_GRAY)
    }

    /// Highlighted/active text
    pub fn text_highlight() -> Style {
        Style::default()
            .fg(NEON_CYAN)
            .add_modifier(Modifier::BOLD)
    }

    /// Error text
    pub fn text_error() -> Style {
        Style::default().fg(RONIN_RED).add_modifier(Modifier::BOLD)
    }

    /// Warning text
    pub fn text_warning() -> Style {
        Style::default().fg(EMBER_ORANGE)
    }

    /// Border style (default)
    pub fn border() -> Style {
        Style::default().fg(STEEL_GRAY)
    }

    /// Border style (focused)
    pub fn border_focused() -> Style {
        Style::default().fg(NEON_CYAN)
    }

    /// Border style (active/selected)
    pub fn border_active() -> Style {
        Style::default()
            .fg(SAKURA_PINK)
            .add_modifier(Modifier::BOLD)
    }

    /// Background style
    pub fn background() -> Style {
        Style::default().bg(VOID_BLACK)
    }

    /// Selected item in list
    pub fn selected() -> Style {
        Style::default()
            .fg(VOID_BLACK)
            .bg(NEON_CYAN)
            .add_modifier(Modifier::BOLD)
    }

    /// Gauge/progress bar filled portion
    pub fn gauge_filled() -> Style {
        Style::default().fg(SAKURA_PINK).bg(SHADOW_GRAY)
    }

    /// Title style
    pub fn title() -> Style {
        Style::default()
            .fg(NEON_CYAN)
            .add_modifier(Modifier::BOLD)
    }

    /// Subtitle style
    pub fn subtitle() -> Style {
        Style::default()
            .fg(SAKURA_PINK)
            .add_modifier(Modifier::ITALIC)
    }
}

/// Style helpers for power profiles
pub mod profile_styles {
    use super::colors::*;
    use ratatui::style::{Modifier, Style};

    pub fn quiet() -> Style {
        Style::default().fg(ZEN_PURPLE).add_modifier(Modifier::BOLD)
    }

    pub fn balanced() -> Style {
        Style::default()
            .fg(BALANCE_BLUE)
            .add_modifier(Modifier::BOLD)
    }

    pub fn performance() -> Style {
        Style::default()
            .fg(EMBER_ORANGE)
            .add_modifier(Modifier::BOLD)
    }
}

/// ASCII art for the Oni mask logo
pub const ONI_MASK: &str = r#"      ▄▄▄▄▄▄▄▄▄▄▄▄▄▄
    ██▀▀          ▀▀██
  ██▀   ▄██▄  ▄██▄   ▀██
 ██     ████  ████     ██
██      ▀▀▀▀  ▀▀▀▀      ██
██    ▄▄          ▄▄    ██
▀██▄▄███          ███▄▄██▀
   ▀▀▀▀            ▀▀▀▀"#;

/// Compact Oni for smaller displays
pub const ONI_COMPACT: &str = r#"
  ╔═══════════╗
  ║ ◉  鬼  ◉ ║
  ║  ╲▄▄▄╱   ║
  ╚═══════════╝
"#;

/// Hachi title banner
pub const HACHI_BANNER: &str = r#"
██╗  ██╗ █████╗  ██████╗██╗  ██╗██╗
██║  ██║██╔══██╗██╔════╝██║  ██║██║
███████║███████║██║     ███████║██║
██╔══██║██╔══██║██║     ██╔══██║██║
██║  ██║██║  ██║╚██████╗██║  ██║██║
╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚═╝
"#;

/// Katana blade shape for battery indicator
pub const KATANA_BLADE: &str = "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━▶";
pub const KATANA_EMPTY: &str = "──────────────────────────────▷";

/// Get the appropriate style for a charge level
pub fn charge_level_style(level: u8) -> Style {
    match level {
        0..=20 => Style::default()
            .fg(colors::RONIN_RED)
            .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
        21..=40 => Style::default().fg(colors::EMBER_ORANGE),
        41..=60 => Style::default().fg(colors::BALANCE_BLUE),
        61..=80 => Style::default().fg(colors::NEON_CYAN),
        _ => Style::default().fg(colors::SAKURA_PINK),
    }
}

/// Get profile-specific color
pub fn profile_color(profile: &crate::daemon::PowerProfile) -> Color {
    match profile {
        crate::daemon::PowerProfile::Quiet => colors::ZEN_PURPLE,
        crate::daemon::PowerProfile::Balanced => colors::BALANCE_BLUE,
        crate::daemon::PowerProfile::Performance => colors::EMBER_ORANGE,
    }
}
