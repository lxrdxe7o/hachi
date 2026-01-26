use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::daemon::{FanCurve, PowerProfile};
use crate::ui::header_art::HACHI_BIG_TEXT;
use crate::ui::theme::{colors, profile_styles, styles};

/// Power profile selector widget
pub struct PowerProfileSelector<'a> {
    current: PowerProfile,
    selected: usize,
    focused: bool,
    title: &'a str,
}

impl<'a> PowerProfileSelector<'a> {
    pub fn new(current: PowerProfile) -> Self {
        Self {
            current,
            selected: current.to_u8() as usize,
            focused: false,
            title: " Power Profile ",
        }
    }

    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }
}

impl Widget for PowerProfileSelector<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = if self.focused {
            styles::border_focused()
        } else {
            styles::border()
        };

        let block = Block::default()
            .title("¹power")
            .title_style(styles::title())
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(border_style);

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 3 || inner.width < 20 {
            return;
        }

        let profiles = [
            (PowerProfile::Quiet, "󰤃  Quiet", "Silent operation", "━━━"),
            (PowerProfile::Balanced, "󰛲  Balanced", "Optimal efficiency", "━━━━━"),
            (PowerProfile::Performance, "󰓅  Performance", "Maximum power", "━━━━━━━"),
        ];

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2); 3])
            .split(inner);

        for (i, (profile, name, desc, power_bar)) in profiles.iter().enumerate() {
            let is_selected = self.selected == i;
            let is_active = self.current == *profile;

            let profile_style = match profile {
                PowerProfile::Quiet => profile_styles::quiet(),
                PowerProfile::Balanced => profile_styles::balanced(),
                PowerProfile::Performance => profile_styles::performance(),
            };

            // Enhanced indicators with better visual distinction
            let indicator = if is_active { "◉" } else { "○" };
            let bracket = if is_selected { "▶" } else { " " };

            // Add power level bar for active profile
            let power_indicator = if is_active {
                Span::styled(format!(" {}", power_bar), profile_style.add_modifier(Modifier::BOLD))
            } else {
                Span::raw("")
            };

            let line = Line::from(vec![
                Span::styled(
                    format!(" {} ", bracket),
                    if is_selected {
                        styles::text_highlight()
                    } else {
                        styles::text_dim()
                    },
                ),
                Span::styled(
                    indicator,
                    if is_active {
                        profile_style.add_modifier(Modifier::BOLD)
                    } else {
                        styles::text_dim()
                    },
                ),
                Span::styled(
                    format!(" {}", name),
                    if is_active || is_selected {
                        profile_style.add_modifier(Modifier::BOLD)
                    } else {
                        styles::text_dim()
                    },
                ),
                power_indicator,
            ]);

            let desc_line = Line::from(vec![
                Span::raw("      "),
                Span::styled(
                    *desc,
                    if is_active {
                        profile_style
                    } else {
                        styles::text_dim()
                    },
                ),
            ]);

            if let Some(chunk) = chunks.get(i) {
                if chunk.height >= 2 {
                    buf.set_line(chunk.x, chunk.y, &line, chunk.width);
                    buf.set_line(chunk.x, chunk.y + 1, &desc_line, chunk.width);
                } else {
                    buf.set_line(chunk.x, chunk.y, &line, chunk.width);
                }
            }
        }
    }
}

/// Battery Katana widget - sword-shaped battery indicator
pub struct BatteryKatana {
    charge_limit: u8,
    focused: bool,
    editing: bool,
}

impl BatteryKatana {
    pub fn new(charge_limit: u8) -> Self {
        Self {
            charge_limit,
            focused: false,
            editing: false,
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn editing(mut self, editing: bool) -> Self {
        self.editing = editing;
        self
    }
}

impl Widget for BatteryKatana {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = if self.editing {
            styles::border_active()
        } else if self.focused {
            styles::border_focused()
        } else {
            styles::border()
        };

        let block = Block::default()
            .title("²battery")
            .title_style(styles::title())
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(border_style);

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 3 || inner.width < 20 {
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(inner);

        // Charge limit label with styled help
        let limit_style = crate::ui::theme::charge_level_style(self.charge_limit);
        let label = if self.editing {
            Line::from(vec![
                Span::styled("  Charge Limit: ", styles::text()),
                Span::styled(format!("{}%", self.charge_limit), limit_style.add_modifier(Modifier::BOLD)),
                Span::styled("  ", styles::text()),
                Span::styled("[←/→]", styles::text_highlight()),
                Span::styled(" adjust", styles::text_dim()),
            ])
        } else {
            Line::from(vec![
                Span::styled("  Charge Limit: ", styles::text()),
                Span::styled(format!("{}%", self.charge_limit), limit_style),
            ])
        };
        buf.set_line(chunks[0].x, chunks[0].y, &label, chunks[0].width);

        // Katana blade visualization with enhanced graphics
        let blade_width = chunks[1].width.saturating_sub(6) as usize;
        let filled = (blade_width * self.charge_limit as usize) / 100;
        let empty = blade_width.saturating_sub(filled);

        // Enhanced blade handle (tsuba + grip)
        let handle = "┃┫";
        // Blade body (filled portion) - thick bold line
        let filled_blade: String = "━".repeat(filled);
        // Empty portion - thin line
        let empty_blade: String = "╌".repeat(empty);
        // Blade tip - sharp arrow
        let tip = "▶";

        let blade_line = Line::from(vec![
            Span::styled(format!("  {}", handle), Style::default().fg(colors::STEEL_GRAY).bold()),
            Span::styled(filled_blade, limit_style.add_modifier(Modifier::BOLD)),
            Span::styled(empty_blade, styles::text_dim()),
            Span::styled(tip, limit_style.add_modifier(Modifier::BOLD)),
        ]);

        buf.set_line(chunks[1].x, chunks[1].y, &blade_line, chunks[1].width);

        // Scale markers with tick marks
        let scale = "   0%        25%        50%        75%       100%";
        let scale_line = Line::from(Span::styled(scale, styles::text_dim()));
        if chunks[2].width > scale.len() as u16 {
            buf.set_line(chunks[2].x, chunks[2].y, &scale_line, chunks[2].width);
        }
    }
}

/// Fan curve visualization widget
pub struct FanCurveGraph<'a> {
    curve: &'a FanCurve,
    selected_point: Option<usize>,
    focused: bool,
    editing: bool,
}

impl<'a> FanCurveGraph<'a> {
    pub fn new(curve: &'a FanCurve) -> Self {
        Self {
            curve,
            selected_point: None,
            focused: false,
            editing: false,
        }
    }

    pub fn selected_point(mut self, point: Option<usize>) -> Self {
        self.selected_point = point;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn editing(mut self, editing: bool) -> Self {
        self.editing = editing;
        self
    }
}

impl Widget for FanCurveGraph<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = if self.editing {
            styles::border_active()
        } else if self.focused {
            styles::border_focused()
        } else {
            styles::border()
        };

        let status = if self.curve.enabled {
            Span::styled("● Enabled", Style::default().fg(colors::NEON_CYAN).bold())
        } else {
            Span::styled("○ Disabled", Style::default().fg(colors::STEEL_GRAY))
        };

        let block = Block::default()
            .title("³fan")
            .title_style(styles::title())
            .title_bottom(Line::from(status).right_aligned())
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(border_style);

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 8 || inner.width < 30 {
            let msg = Paragraph::new("Expand for graph")
                .style(styles::text_dim())
                .alignment(Alignment::Center);
            msg.render(inner, buf);
            return;
        }

        // Graph dimensions with padding for labels
        let graph_height = inner.height.saturating_sub(3) as usize;
        let graph_width = inner.width.saturating_sub(7) as usize;

        let graph_area = Rect {
            x: inner.x + 6,
            y: inner.y,
            width: graph_width as u16,
            height: graph_height as u16,
        };

        // Draw subtle grid lines first (behind everything)
        draw_grid(buf, &graph_area);

        // Y-axis labels (fan speed %) with decorative line
        for i in 0..=4 {
            let y = inner.y + (graph_height as u16 * i / 4);
            let label = format!("{:>3}%", 100 - (i * 25));
            buf.set_string(inner.x, y, &label, styles::text_dim());
            // Tick mark
            buf.set_string(inner.x + 4, y, "╴", styles::text_dim());
        }

        // X-axis labels (temperature °C)
        let x_labels = ["30°", "50°", "70°", "90°"];
        for (i, label) in x_labels.iter().enumerate() {
            let x = graph_area.x + (graph_width as u16 * i as u16 / 3);
            let y = inner.y + inner.height - 2;
            buf.set_string(x, y, label, styles::text_dim());
        }

        // Collect points for curve drawing
        let points: Vec<(f32, f32)> = self.curve.cpu_curve.iter().map(|point| {
            let x_ratio = (point.temp.saturating_sub(30) as f32) / 70.0;
            let x = graph_area.x as f32 + (graph_area.width as f32 * x_ratio);
            let y_ratio = 1.0 - (point.speed as f32 / 100.0);
            let y = graph_area.y as f32 + (graph_area.height as f32 * y_ratio);
            (x, y)
        }).collect();

        // Draw smooth interpolated curve with gradient
        if points.len() >= 2 {
            draw_smooth_curve(buf, &points, &graph_area, self.focused || self.editing);
        }

        // Draw control points on top of the curve (larger, more visible)
        for (i, point) in self.curve.cpu_curve.iter().enumerate() {
            let x_ratio = (point.temp.saturating_sub(30) as f32) / 70.0;
            let x = graph_area.x + (graph_area.width as f32 * x_ratio) as u16;
            let y_ratio = 1.0 - (point.speed as f32 / 100.0);
            let y = graph_area.y + (graph_area.height as f32 * y_ratio) as u16;

            if x < graph_area.right() && y < graph_area.bottom() {
                let (symbol, style) = if self.selected_point == Some(i) {
                    if self.editing {
                        // Editing: large pulsing red diamond
                        ("◆", styles::graph_point_editing())
                    } else {
                        // Selected: pink diamond
                        ("◆", styles::graph_point_selected())
                    }
                } else {
                    // Normal: cyan circle
                    ("●", styles::graph_point())
                };
                buf.set_string(x, y, symbol, style);

                // Draw point value label for selected point
                if self.selected_point == Some(i) {
                    let label = format!("{}°:{}%", point.temp, point.speed);
                    let label_x = if x + label.len() as u16 + 2 < graph_area.right() {
                        x + 2
                    } else {
                        x.saturating_sub(label.len() as u16 + 1)
                    };
                    let label_y = if y > graph_area.y { y - 1 } else { y + 1 };
                    if label_y < graph_area.bottom() {
                        buf.set_string(label_x, label_y, &label, styles::text_highlight());
                    }
                }
            }
        }

        // Help text with styling
        let help = if self.editing {
            Line::from(vec![
                Span::styled("[↑↓]", styles::text_highlight()),
                Span::styled(" Speed  ", styles::text_dim()),
                Span::styled("[←→]", styles::text_highlight()),
                Span::styled(" Temp  ", styles::text_dim()),
                Span::styled("[Enter]", styles::text_highlight()),
                Span::styled(" Confirm", styles::text_dim()),
            ])
        } else if self.focused {
            Line::from(vec![
                Span::styled("[Enter]", styles::text_highlight()),
                Span::styled(" Edit  ", styles::text_dim()),
                Span::styled("[Tab]", styles::text_highlight()),
                Span::styled(" Next", styles::text_dim()),
            ])
        } else {
            Line::from("")
        };
        let help_y = inner.y + inner.height - 1;
        buf.set_line(inner.x + 6, help_y, &help, inner.width.saturating_sub(6));
    }
}

/// Draw a subtle grid in the graph area
fn draw_grid(buf: &mut Buffer, area: &Rect) {
    let grid_style = Style::default().fg(colors::SHADOW_GRAY);

    // Horizontal grid lines at 25% intervals
    for i in 1..4 {
        let y = area.y + (area.height * i / 4);
        for x in area.x..area.right() {
            if x % 2 == 0 {  // Dotted line effect
                buf.set_string(x, y, "·", grid_style);
            }
        }
    }

    // Vertical grid lines at temperature intervals
    for i in 1..4 {
        let x = area.x + (area.width * i / 4);
        for y in area.y..area.bottom() {
            if y % 2 == 0 {  // Dotted line effect
                buf.set_string(x, y, "·", grid_style);
            }
        }
    }
}

/// Draw a smooth curve through the points using Catmull-Rom interpolation
fn draw_smooth_curve(buf: &mut Buffer, points: &[(f32, f32)], area: &Rect, is_active: bool) {
    if points.len() < 2 {
        return;
    }

    // Gradient colors: Cyan -> Pink (more vibrant when active)
    let (start_r, start_g, start_b) = if is_active { (60, 220, 255) } else { (60, 180, 200) };
    let (end_r, end_g, end_b) = if is_active { (255, 60, 120) } else { (200, 60, 100) };

    // Generate interpolated points using Catmull-Rom splines
    let mut curve_points: Vec<(f32, f32)> = Vec::new();

    for i in 0..points.len() - 1 {
        let p0 = if i == 0 { points[0] } else { points[i - 1] };
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = if i + 2 < points.len() { points[i + 2] } else { points[i + 1] };

        // Generate points along the spline segment
        let steps = ((p2.0 - p1.0).abs() as usize).max(10);
        for step in 0..=steps {
            let t = step as f32 / steps as f32;
            let point = catmull_rom(p0, p1, p2, p3, t);
            curve_points.push(point);
        }
    }

    // Draw the curve with gradient coloring and thick characters
    let total_points = curve_points.len();
    for (i, window) in curve_points.windows(2).enumerate() {
        let (x0, y0) = window[0];
        let (x1, y1) = window[1];

        // Calculate gradient color based on position along curve
        let t = i as f32 / total_points as f32;
        let r = (start_r as f32 * (1.0 - t) + end_r as f32 * t) as u8;
        let g = (start_g as f32 * (1.0 - t) + end_g as f32 * t) as u8;
        let b = (start_b as f32 * (1.0 - t) + end_b as f32 * t) as u8;
        let color = Color::Rgb(r, g, b);

        draw_thick_line(buf, x0 as u16, y0 as u16, x1 as u16, y1 as u16, color, area);
    }
}

/// Catmull-Rom spline interpolation for smooth curves
fn catmull_rom(p0: (f32, f32), p1: (f32, f32), p2: (f32, f32), p3: (f32, f32), t: f32) -> (f32, f32) {
    let t2 = t * t;
    let t3 = t2 * t;

    // Catmull-Rom basis functions
    let x = 0.5 * ((2.0 * p1.0) +
                   (-p0.0 + p2.0) * t +
                   (2.0 * p0.0 - 5.0 * p1.0 + 4.0 * p2.0 - p3.0) * t2 +
                   (-p0.0 + 3.0 * p1.0 - 3.0 * p2.0 + p3.0) * t3);

    let y = 0.5 * ((2.0 * p1.1) +
                   (-p0.1 + p2.1) * t +
                   (2.0 * p0.1 - 5.0 * p1.1 + 4.0 * p2.1 - p3.1) * t2 +
                   (-p0.1 + 3.0 * p1.1 - 3.0 * p2.1 + p3.1) * t3);

    (x, y)
}

/// Draw a thick line using bold box-drawing characters
fn draw_thick_line(buf: &mut Buffer, x0: u16, y0: u16, x1: u16, y1: u16, color: Color, area: &Rect) {
    let dx = (x1 as i32 - x0 as i32).abs();
    let dy = (y1 as i32 - y0 as i32).abs();
    let sx = if x0 < x1 { 1i32 } else { -1i32 };
    let sy = if y0 < y1 { 1i32 } else { -1i32 };
    let mut err = dx - dy;
    let mut x = x0 as i32;
    let mut y = y0 as i32;

    let style = Style::default().fg(color).add_modifier(Modifier::BOLD);

    loop {
        if x >= area.x as i32 && y >= area.y as i32
           && x < area.right() as i32 && y < area.bottom() as i32 {
            // Choose character based on direction for better visual continuity
            let ch = if dx == 0 {
                '┃'  // Vertical thick line
            } else if dy == 0 {
                '━'  // Horizontal thick line
            } else {
                // Calculate local slope for this segment
                let local_dx = (x1 as i32 - x).abs();
                let local_dy = (y1 as i32 - y).abs();

                if local_dx > local_dy * 2 {
                    '━'  // Mostly horizontal
                } else if local_dy > local_dx * 2 {
                    '┃'  // Mostly vertical
                } else if (sx > 0 && sy > 0) || (sx < 0 && sy < 0) {
                    '╲'  // Diagonal down-right or up-left
                } else {
                    '╱'  // Diagonal up-right or down-left
                }
            };

            buf.set_string(x as u16, y as u16, ch.to_string(), style);
        }

        if x == x1 as i32 && y == y1 as i32 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

/// Status bar widget showing connection status and errors
pub struct StatusBar<'a> {
    connected: bool,
    message: Option<&'a str>,
}

impl<'a> StatusBar<'a> {
    pub fn new(connected: bool) -> Self {
        Self {
            connected,
            message: None,
        }
    }

    pub fn message(mut self, msg: &'a str) -> Self {
        self.message = Some(msg);
        self
    }
}

impl Widget for StatusBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Background
        buf.set_style(area, Style::default().bg(colors::SHADOW_GRAY));

        // Connection status
        let (status_icon, status_style) = if self.connected {
            ("● Connected", Style::default().fg(colors::NEON_CYAN))
        } else {
            ("○ Disconnected", Style::default().fg(colors::RONIN_RED))
        };

        buf.set_string(area.x + 1, area.y, status_icon, status_style);

        // Message (if any)
        if let Some(msg) = self.message {
            let msg_x = area.x + 20;
            let style = if msg.contains("Error") {
                styles::text_error()
            } else {
                styles::text_warning()
            };
            let available_width = area.width.saturating_sub(21) as usize;
            let truncated = if msg.len() > available_width {
                format!("{}...", &msg[..available_width.saturating_sub(3)])
            } else {
                msg.to_string()
            };
            buf.set_string(msg_x, area.y, &truncated, style);
        }

        // Keybinds hint on right
        let hint = " q: quit  s: sakura  tab: cycle  ?: help ";
        let hint_x = area.right().saturating_sub(hint.len() as u16 + 1);
        buf.set_string(hint_x, area.y, hint, styles::text_dim());
    }
}

/// Header widget with Oni logo and title
pub struct Header {
    compact: bool,
}

impl Header {
    pub fn new() -> Self {
        Self { compact: false }
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Header {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 2 || area.width < 40 {
            return;
        }

        // Simple header with title on the left
        let left_padding = 3u16;
        let text_x = area.x + left_padding;

        let big_text_height = HACHI_BIG_TEXT.len() as u16;
        // Center the block text vertically in the header area
        let title_y = area.y + (area.height.saturating_sub(big_text_height)) / 2;

        // Gradient start/end colors: Cyan -> Pink
        let (r1, g1, b1) = (60, 203, 225);  // Neon Cyan
        let (r2, g2, b2) = (255, 0, 85);    // Sakura Pink

        // Render Big Text with gradient
        for (row, line) in HACHI_BIG_TEXT.iter().enumerate() {
            let y = title_y + row as u16;
            if y >= area.y + area.height { break; }

            let line_len = line.chars().count();
            for (col, ch) in line.chars().enumerate() {
                let x = text_x + col as u16;
                if x >= area.x + area.width { break; }

                if ch != ' ' {
                    // Linear interpolation for gradient based on column
                    let t = col as f32 / line_len as f32;
                    let r = (r1 as f32 * (1.0 - t) + r2 as f32 * t) as u8;
                    let g = (g1 as f32 * (1.0 - t) + g2 as f32 * t) as u8;
                    let b = (b1 as f32 * (1.0 - t) + b2 as f32 * t) as u8;

                    if let Some(cell) = buf.cell_mut((x, y)) {
                        cell.set_char(ch);
                        cell.set_fg(Color::Rgb(r, g, b));
                    }
                }
            }
        }

        // Render subtitle to the right of the big text, vertically centered
        let subtitle_x = text_x + 40; // After the HACHI text
        let subtitle_y = area.y + area.height / 2; // Center vertically

        if subtitle_x < area.x + area.width && subtitle_y < area.y + area.height {
            let subtitle = Line::from(vec![
                Span::styled("蜂 ", Style::default().fg(Color::Rgb(255, 200, 50)).add_modifier(Modifier::BOLD)),
                Span::styled("ASUS ROG Control Center", styles::text_dim()),
            ]);
            buf.set_line(subtitle_x, subtitle_y, &subtitle, area.width.saturating_sub(subtitle_x));
        }
    }
}

/// Help popup widget
pub struct HelpPopup;

impl Widget for HelpPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Semi-transparent background
        buf.set_style(area, Style::default().bg(colors::SHADOW_GRAY));

        let block = Block::default()
            .title("⁴help")
            .title_style(styles::title())
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(styles::border_focused());

        let inner = block.inner(area);
        block.render(area, buf);

        let help_text = vec![
            Line::from(vec![
                Span::styled("Navigation", styles::text_highlight()),
            ]),
            Line::from("  H / L (Shift)   - Cycle panels"),
            Line::from("  Tab / Shift+Tab - Cycle panels"),
            Line::from("  k / j           - Select option"),
            Line::from("  Enter           - Confirm / Edit"),
            Line::from("  Esc             - Cancel / Back"),
            Line::from(""),
            Line::from(vec![Span::styled("Controls", styles::text_highlight())]),
            Line::from("  ← / →           - Adjust values"),
            Line::from("  Space           - Toggle"),
            Line::from(""),
            Line::from(vec![Span::styled("Global", styles::text_highlight())]),
            Line::from("  q               - Quit"),
            Line::from("  r               - Refresh state"),
            Line::from("  ?               - Toggle help"),
        ];

        let para = Paragraph::new(help_text)
            .style(styles::text())
            .alignment(Alignment::Left);
        para.render(inner, buf);
    }
}
