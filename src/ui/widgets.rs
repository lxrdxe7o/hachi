use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::daemon::{FanCurve, PowerProfile};
use crate::ui::header_art::{header_color, HACHI_BIG_TEXT, HEADER_ART, HEADER_COLS};
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
            .border_type(BorderType::Plain)
            .border_style(border_style);

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 3 || inner.width < 20 {
            return;
        }

        let profiles = [
            (PowerProfile::Quiet, "󰤃  Quiet", "Silent operation"),
            (PowerProfile::Balanced, "󰛲  Balanced", "Optimal efficiency"),
            (PowerProfile::Performance, "󰓅  Performance", "Maximum power"),
        ];

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2); 3])
            .split(inner);

        for (i, (profile, name, desc)) in profiles.iter().enumerate() {
            let is_selected = self.selected == i;
            let is_active = self.current == *profile;

            let profile_style = match profile {
                PowerProfile::Quiet => profile_styles::quiet(),
                PowerProfile::Balanced => profile_styles::balanced(),
                PowerProfile::Performance => profile_styles::performance(),
            };

            let indicator = if is_active { "◉" } else { "○" };
            let bracket = if is_selected { "▶" } else { " " };

            let line = Line::from(vec![
                Span::styled(
                    format!(" {} ", bracket),
                    if is_selected {
                        styles::text_highlight()
                    } else {
                        styles::text_dim()
                    },
                ),
                Span::styled(indicator, profile_style),
                Span::styled(format!(" {}", name), profile_style),
            ]);

            let desc_line = Line::from(vec![
                Span::raw("      "),
                Span::styled(*desc, styles::text_dim()),
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
            .border_type(BorderType::Plain)
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

        // Charge limit label
        let limit_style = crate::ui::theme::charge_level_style(self.charge_limit);
        let label = Line::from(vec![
            Span::styled("  Charge Limit: ", styles::text()),
            Span::styled(format!("{}%", self.charge_limit), limit_style),
            if self.editing {
                Span::styled(" [←/→ to adjust]", styles::text_dim())
            } else {
                Span::raw("")
            },
        ]);
        buf.set_line(chunks[0].x, chunks[0].y, &label, chunks[0].width);

        // Katana blade visualization
        let blade_width = chunks[1].width.saturating_sub(4) as usize;
        let filled = (blade_width * self.charge_limit as usize) / 100;
        let empty = blade_width.saturating_sub(filled);

        // Blade handle
        let handle = "┫";
        // Blade body (filled portion)
        let filled_blade: String = "━".repeat(filled);
        // Empty portion
        let empty_blade: String = "─".repeat(empty);
        // Blade tip
        let tip = "▶";

        let blade_line = Line::from(vec![
            Span::styled(format!("  {}", handle), styles::text_dim()),
            Span::styled(filled_blade, limit_style),
            Span::styled(empty_blade, styles::text_dim()),
            Span::styled(tip, limit_style),
        ]);

        buf.set_line(chunks[1].x, chunks[1].y, &blade_line, chunks[1].width);

        // Scale markers
        let scale = "  0%         25%         50%         75%        100%";
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
            "● Enabled"
        } else {
            "○ Disabled"
        };

        let block = Block::default()
            .title("³fan")
            .title_style(styles::title())
            .title_bottom(Line::from(status).right_aligned())
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(border_style);

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 8 || inner.width < 30 {
            // Too small, show minimal info
            let msg = Paragraph::new("Expand for graph")
                .style(styles::text_dim())
                .alignment(Alignment::Center);
            msg.render(inner, buf);
            return;
        }

        // Graph dimensions
        let graph_height = inner.height.saturating_sub(3) as usize;
        let graph_width = inner.width.saturating_sub(6) as usize;

        // Y-axis labels (fan speed %)
        for i in 0..=4 {
            let y = inner.y + (graph_height as u16 * i / 4);
            let label = format!("{:>3}%", 100 - (i * 25));
            let style = styles::text_dim();
            buf.set_string(inner.x, y, &label, style);
        }

        // X-axis labels (temperature °C)
        let x_labels = ["30°", "50°", "70°", "90°"];
        for (i, label) in x_labels.iter().enumerate() {
            let x = inner.x + 5 + (graph_width as u16 * i as u16 / 3);
            let y = inner.y + inner.height - 2;
            buf.set_string(x, y, label, styles::text_dim());
        }

        // Draw grid lines
        let graph_area = Rect {
            x: inner.x + 5,
            y: inner.y,
            width: graph_width as u16,
            height: graph_height as u16,
        };

        // Draw curve points
        for (i, point) in self.curve.cpu_curve.iter().enumerate() {
            // Map temperature (30-100) to x position
            let x_ratio = (point.temp.saturating_sub(30) as f32) / 70.0;
            let x = graph_area.x + (graph_area.width as f32 * x_ratio) as u16;

            // Map speed (0-100) to y position (inverted)
            let y_ratio = 1.0 - (point.speed as f32 / 100.0);
            let y = graph_area.y + (graph_area.height as f32 * y_ratio) as u16;

            if x < graph_area.right() && y < graph_area.bottom() {
                let (symbol, style) = if self.selected_point == Some(i) {
                    if self.editing {
                        ("◆", Style::default().fg(colors::RONIN_RED).bold().add_modifier(Modifier::SLOW_BLINK))
                    } else {
                        ("◆", Style::default().fg(colors::SAKURA_PINK).bold())
                    }
                } else {
                    ("●", Style::default().fg(colors::NEON_CYAN))
                };
                buf.set_string(x, y, symbol, style);
            }

            // Draw line to next point
            if i < self.curve.cpu_curve.len() - 1 {
                let next = &self.curve.cpu_curve[i + 1];
                let next_x_ratio = (next.temp.saturating_sub(30) as f32) / 70.0;
                let next_x = graph_area.x + (graph_area.width as f32 * next_x_ratio) as u16;
                let next_y_ratio = 1.0 - (next.speed as f32 / 100.0);
                let next_y = graph_area.y + (graph_area.height as f32 * next_y_ratio) as u16;

                // Simple line drawing between points
                draw_line(buf, x, y, next_x, next_y, colors::NEON_CYAN);
            }
        }

        // Help text
        let help = if self.editing {
            "[↑↓] Speed  [←→] Temp  [Enter] Confirm"
        } else if self.focused {
            "[Enter] Edit  [Tab] Next"
        } else {
            ""
        };
        let help_y = inner.y + inner.height - 1;
        buf.set_string(inner.x + 5, help_y, help, styles::text_dim());
    }
}

/// Simple line drawing helper (Bresenham's algorithm simplified)
fn draw_line(buf: &mut Buffer, x0: u16, y0: u16, x1: u16, y1: u16, color: ratatui::style::Color) {
    let dx = (x1 as i32 - x0 as i32).abs();
    let dy = (y1 as i32 - y0 as i32).abs();
    let sx = if x0 < x1 { 1i32 } else { -1i32 };
    let sy = if y0 < y1 { 1i32 } else { -1i32 };
    let mut err = dx - dy;
    let mut x = x0 as i32;
    let mut y = y0 as i32;

    let style = Style::default().fg(color);
    let char = if dx > dy * 2 {
        '─'
    } else if dy > dx * 2 {
        '│'
    } else if (sx > 0 && sy > 0) || (sx < 0 && sy < 0) {
        '╲'
    } else {
        '╱'
    };

    loop {
        if x >= 0 && y >= 0 {
            buf.set_string(x as u16, y as u16, char.to_string(), style);
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
        let hint = " q: quit  tab: cycle  ?: help ";
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

        // Render per-character colored header art using braille characters
        // Skip the first 3 solid lines and start from the interesting part
        let skip_lines = 3;
        let max_art_rows = area.height as usize;
        
        // Art starts with left padding
        let art_width = HEADER_COLS as u16;
        let left_padding = 2u16;
        let top_padding = 1u16;
        let start_x = area.x + left_padding;
        let start_y = area.y + top_padding;

        // Render the art lines (starting from skip_lines, limited to max_art_rows)
        let art_lines: Vec<_> = HEADER_ART.iter().skip(skip_lines).take(max_art_rows).collect();
        let _rendered_rows = art_lines.len();
        
        for (row, line) in art_lines.iter().enumerate() {
            let y = start_y + row as u16;
            if y >= area.y + area.height {
                break;
            }

            // Iterate over characters in the line
            for (col, ch) in line.chars().enumerate() {
                let x = start_x + col as u16;
                if x >= area.x + area.width {
                    break;
                }

                // Adjust color index to account for skipped lines
                let original_row = row + skip_lines;
                let idx = original_row * HEADER_COLS + col;
                let color = header_color(idx);
                
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_char(ch);
                    cell.set_fg(color);
                }
            }
        }

        // Render title and subtitle to the RIGHT of the art
        // Render BIG title and subtitle to the RIGHT of the art
        let text_x = start_x + art_width + 8; // 8 chars padding from art
        if text_x < area.x + area.width {
            let big_text_height = HACHI_BIG_TEXT.len() as u16;
            // Center the block text vertically in the header area
            let title_y = area.y + (area.height.saturating_sub(big_text_height + 2) / 2);
            
            // Gradient start/end colors
            let (r1, g1, b1) = (60, 203, 225); // Neon Cyan (Muted)
            let (r2, g2, b2) = (255, 0, 85);   // Sakura Pink
            
            // Render Big Text
            for (row, line) in HACHI_BIG_TEXT.iter().enumerate() {
                let y = title_y + row as u16;
                if y >= area.y + area.height { break; }
                
                let line_len = line.chars().count();
                for (col, ch) in line.chars().enumerate() {
                    let x = text_x + col as u16;
                    if x >= area.x + area.width { break; }
                    
                    // Box drawing characters don't have spaces, so we can check easily
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
            
            // Render Japanese character and subtitle below the big text
            let subtitle_y = title_y + big_text_height + 2; // 2 rows spacing
            if subtitle_y < area.y + area.height {
                 // Align with big text
                 let subtitle = Line::from(vec![
                    Span::styled("  八  ", Style::default().fg(colors::SAKURA_PINK).add_modifier(Modifier::BOLD)),
                    Span::styled("ASUS Ronin Control Center", styles::subtitle().fg(colors::SAKURA_PINK)),
                ]);
                buf.set_line(text_x, subtitle_y, &subtitle, area.width.saturating_sub(text_x));
            }
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
            .border_type(BorderType::Plain)
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
