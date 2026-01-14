<p align="center">
  <img src="https://raw.githubusercontent.com/x3ro/hachi/main/assets/logo.png" alt="Hachi Logo" width="400"/>
</p>

<h1 align="center">八 HACHI</h1>
<p align="center">
  <em>ASUS ROG Laptop Control Center for Linux</em>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#keybindings">Keybindings</a> •
  <a href="#requirements">Requirements</a>
</p>

---

## Overview

**Hachi** (八, Japanese for "eight") is a beautiful, modern TUI (Terminal User Interface) application for managing ASUS ROG laptops on Linux. It provides an intuitive interface for controlling power profiles, battery charge limits, and custom fan curves through the `asusd` daemon.

Built with a cyberpunk-inspired aesthetic featuring sakura particle effects, gradient text, and a minimalist design language.

## Features

- **🎮 Power Profile Management** - Switch between Quiet, Balanced, and Performance modes
- **🔋 Battery Charge Limiter** - Set custom charge limits to prolong battery lifespan
- **🌡️ Custom Fan Curves** - Fine-tune fan behavior with an interactive curve editor
- **🌸 Sakura Particle Effects** - Beautiful animated cherry blossom petals floating across the screen
- **🎨 Gradient Header** - Eye-catching "HACHI" title with cyan-to-pink gradient
- **⌨️ Vim-style Navigation** - Familiar keybindings for efficient control

## Screenshots

_Coming soon_

## Installation

### Prerequisites

- **Rust** (1.70+) - Install via [rustup](https://rustup.rs/)
- **asusd** - ASUS Linux daemon ([asus-linux.org](https://asus-linux.org/))

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/hachi.git
cd hachi

# Build and install
cargo build --release

# Run
./target/release/hachi
```

### Arch Linux (AUR)

```bash
# Coming soon
yay -S hachi
```

## Usage

Simply run the application:

```bash
hachi
```

The TUI will display:

- **Header** - Animated logo with gradient title
- **Power Profile Panel** - Current and available power modes
- **Battery Panel** - Charge limit slider (0-100%)
- **Fan Curve Panel** - Interactive temperature/speed graph

## Keybindings

| Key                 | Action                            |
| ------------------- | --------------------------------- |
| `1` / `2` / `3`     | Focus Power / Battery / Fan panel |
| `Tab` / `Shift+Tab` | Cycle through panels              |
| `H` / `L`           | Previous / Next panel (Vim-style) |
| `j` / `k`           | Navigate options                  |
| `Enter` / `Space`   | Confirm / Edit                    |
| `←` / `→`           | Adjust values                     |
| `Esc`               | Cancel / Exit edit mode           |
| `r`                 | Refresh state from daemon         |
| `?`                 | Toggle help                       |
| `q`                 | Quit                              |

## Architecture

```
src/
├── main.rs         # Entry point and event loop
├── app.rs          # Application state and logic
├── daemon.rs       # D-Bus communication with asusd
├── error.rs        # Error types
└── ui/
    ├── mod.rs      # UI module exports
    ├── widgets.rs  # Custom ratatui widgets
    ├── theme.rs    # Color palette and styles
    ├── effects.rs  # Sakura particles and animations
    └── header_art.rs # Logo art and gradient colors
```

## Technology Stack

| Technology                                             | Purpose                              |
| ------------------------------------------------------ | ------------------------------------ |
| [Rust](https://www.rust-lang.org/)                     | Systems programming language         |
| [Ratatui](https://ratatui.rs/)                         | Terminal UI framework                |
| [Crossterm](https://github.com/crossterm-rs/crossterm) | Cross-platform terminal manipulation |
| [Tokio](https://tokio.rs/)                             | Async runtime                        |
| [zbus](https://docs.rs/zbus/)                          | D-Bus communication                  |
| [tachyonfx](https://docs.rs/tachyonfx/)                | Terminal visual effects              |

## Requirements

- Linux with ASUS ROG laptop
- `asusd` daemon running (provides D-Bus interface)
- Terminal with true color support (recommended)
- Nerd Font for icons (optional but recommended)

## Configuration

Hachi reads from the `asusd` configuration. No additional configuration files are needed.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [asus-linux.org](https://asus-linux.org/) for the amazing `asusd` daemon
- The Rust community for excellent libraries
- Inspired by cyberpunk aesthetics and Japanese design

---

<p align="center">
  <strong>八 HACHI</strong> - Made with ❤️ for ASUS ROG Linux users
</p>
