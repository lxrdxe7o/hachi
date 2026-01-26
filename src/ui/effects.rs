use std::time::Duration;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
};
use tachyonfx::{fx, Duration as FxDuration, Effect, Shader};

use super::theme::colors;

/// Sakura petal characters for particle effects
const SAKURA_CHARS: [char; 6] = ['❀', '✿', '❁', '✾', '❃', '✤'];

/// Create a cyberpunk glitch effect for mode transitions
pub fn glitch_burst(duration_ms: u32) -> Effect {
    let quarter = duration_ms / 4;

    // RGB split / color shift sequence
    fx::sequence(&[
        fx::fade_to_fg(colors::NEON_CYAN, quarter),
        fx::fade_to_fg(colors::SAKURA_PINK, quarter),
        fx::fade_to_fg(colors::RONIN_RED, quarter),
        fx::fade_to_fg(colors::GHOST_WHITE, quarter),
    ])
}

/// Create a horizontal scan line effect
pub fn scan_line(duration_ms: u32) -> Effect {
    fx::sweep_in(
        fx::Direction::LeftToRight,
        1u16,
        1u16,
        colors::NEON_CYAN,
        duration_ms,
    )
}

/// Create a fade-in effect for UI elements
pub fn fade_in(duration_ms: u32) -> Effect {
    fx::fade_from_fg(colors::VOID_BLACK, duration_ms)
}

/// Create a pulse effect for selected items
pub fn pulse_highlight(color: Color) -> Effect {
    fx::ping_pong(fx::fade_to_fg(color, 600u32))
}

/// Create border glow effect
pub fn border_glow(color: Color, duration_ms: u32) -> Effect {
    fx::ping_pong(fx::fade_to_fg(color, duration_ms))
}

/// Create an animated border pulse that cycles through colors
pub fn border_pulse_cycle() -> Effect {
    fx::ping_pong(fx::sequence(&[
        fx::fade_to_fg(colors::NEON_CYAN, 400u32),
        fx::fade_to_fg(colors::SAKURA_PINK, 400u32),
    ]))
}

/// Create a subtle shimmer effect for focused borders
pub fn border_shimmer(duration_ms: u32) -> Effect {
    fx::sequence(&[
        fx::fade_to_fg(Color::Rgb(80, 220, 245), duration_ms / 2),  // Bright cyan
        fx::fade_to_fg(colors::NEON_CYAN, duration_ms / 2),         // Back to normal
    ])
}

/// Create a "power up" effect for profile changes
pub fn power_surge(profile_color: Color) -> Effect {
    fx::sequence(&[
        // Initial flash
        fx::fade_to_fg(Color::White, 50u32),
        // Settle to profile color
        fx::fade_to_fg(profile_color, 200u32),
        // Brief dissolve
        fx::dissolve(100u32),
        // Return to normal
        fx::fade_to_fg(colors::GHOST_WHITE, 300u32),
    ])
}

/// Create charging animation for battery
pub fn battery_charge_pulse(level: u8) -> Effect {
    let color = match level {
        0..=20 => colors::RONIN_RED,
        21..=50 => colors::EMBER_ORANGE,
        51..=80 => colors::NEON_CYAN,
        _ => colors::SAKURA_PINK,
    };

    fx::ping_pong(fx::fade_to_fg(color, 800u32))
}

/// Create a "data stream" effect for fan curves
pub fn data_stream() -> Effect {
    fx::sweep_in(
        fx::Direction::DownToUp,
        1u16,
        1u16,
        colors::NEON_CYAN,
        500u32,
    )
}

/// Effect manager to track and update active effects
pub struct EffectManager {
    effects: Vec<(String, Effect, Rect)>,
}

impl EffectManager {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    /// Add a named effect for a specific area
    pub fn add(&mut self, name: impl Into<String>, effect: Effect, area: Rect) {
        let name = name.into();
        // Remove any existing effect with same name
        self.effects.retain(|(n, _, _)| n != &name);
        self.effects.push((name, effect, area));
    }

    /// Remove an effect by name
    pub fn remove(&mut self, name: &str) {
        self.effects.retain(|(n, _, _)| n != name);
    }

    /// Clear all effects
    pub fn clear(&mut self) {
        self.effects.clear();
    }

    /// Process all effects for a frame
    pub fn process(&mut self, duration: Duration, buf: &mut Buffer, area: Rect) {
        let fx_duration = FxDuration::from_millis(duration.as_millis() as u32);

        // Remove completed effects and process active ones
        self.effects.retain_mut(|(_, effect, effect_area)| {
            // Only process if effect area intersects with render area
            if area.intersects(*effect_area) {
                effect.process(fx_duration, buf, *effect_area);
                !effect.done()
            } else {
                true // Keep effects outside current view
            }
        });
    }

    /// Check if any effects are active
    pub fn has_active_effects(&self) -> bool {
        !self.effects.is_empty()
    }

    /// Trigger a glitch effect on profile change
    pub fn trigger_profile_glitch(&mut self, area: Rect, profile_color: Color) {
        self.add("profile_glitch", power_surge(profile_color), area);
        self.add("scan", scan_line(300), area);
    }

    /// Trigger battery update effect
    pub fn trigger_battery_update(&mut self, area: Rect, level: u8) {
        self.add("battery_pulse", battery_charge_pulse(level), area);
    }

    /// Trigger border glow animation for focused panel
    pub fn trigger_border_glow(&mut self, name: &str, area: Rect, color: Color) {
        self.add(name, border_glow(color, 800), area);
    }

    /// Trigger cycling border animation
    pub fn trigger_border_cycle(&mut self, name: &str, area: Rect) {
        self.add(name, border_pulse_cycle(), area);
    }
}

impl Default for EffectManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Custom shader for rendering sakura particles in background
pub struct SakuraShader {
    particles: Vec<SakuraParticle>,
    width: u16,
    height: u16,
}

struct SakuraParticle {
    x: f32,
    y: f32,
    char_idx: usize,
    speed: f32,
    drift: f32,
    alpha: f32,
}

impl SakuraShader {
    pub fn new(width: u16, height: u16, density: usize) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let particles = (0..density)
            .map(|_| SakuraParticle {
                x: rng.gen_range(0.0..width as f32),
                y: rng.gen_range(0.0..height as f32),
                char_idx: rng.gen_range(0..SAKURA_CHARS.len()),
                speed: rng.gen_range(0.1..0.4),
                drift: rng.gen_range(-0.2..0.2),
                alpha: rng.gen_range(0.3..1.0),
            })
            .collect();

        Self {
            particles,
            width,
            height,
        }
    }

    /// Update particle positions
    pub fn update(&mut self, delta: Duration) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let dt = delta.as_secs_f32();

        for particle in &mut self.particles {
            // Move down with drift
            particle.y += particle.speed * dt * 10.0;
            particle.x += particle.drift * dt * 5.0;

            // Wrap around screen
            if particle.y > self.height as f32 {
                particle.y = -1.0;
                particle.x = rng.gen_range(0.0..self.width as f32);
                particle.alpha = rng.gen_range(0.3..1.0);
            }
            if particle.x < 0.0 {
                particle.x = self.width as f32 - 1.0;
            } else if particle.x >= self.width as f32 {
                particle.x = 0.0;
            }
        }
    }

    /// Render particles to buffer
    pub fn render(&self, buf: &mut Buffer, area: Rect) {
        for particle in &self.particles {
            let x = area.x + particle.x as u16;
            let y = area.y + particle.y as u16;

            if x < area.right() && y < area.bottom() && x >= area.x && y >= area.y {
                let ch = SAKURA_CHARS[particle.char_idx];
                // Vary pink based on alpha
                let intensity = (particle.alpha * 255.0) as u8;
                let color = Color::Rgb(255, intensity / 3, intensity / 2);

                if let Some(cell) = buf.cell_mut((x, y)) {
                    // Only render on empty/background cells
                    if cell.symbol() == " " {
                        cell.set_char(ch).set_fg(color);
                    }
                }
            }
        }
    }

    /// Resize the shader area
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }
}
