//! Bee-themed header art for HACHI (蜂 = bee in Japanese)
//! Contains braille character art with per-character RGB colors

use ratatui::style::Color;

/// Header art lines - Cute bee design using braille characters
/// Each line is 48 visible characters (including leading/trailing space)
pub const HEADER_ART: &[&str] = &[
    "                                                ",
    "                                                ",
    "                                                ",
    "              ⣀⣀⣤⣤⣤⣤⣤⣤⣀⣀                        ",
    "          ⣀⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣦⣄                    ",
    "        ⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄                  ",
    "       ⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦                 ",
    "   ⢀⣤⣤⣾⣿⣿⣿⣿⣿⣿⡿⠛⠛⠛⠛⠛⠛⢿⣿⣿⣿⣿⣿⣿⣿⣷⣤⣤⡀             ",
    "  ⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠀⠀⣀⣀⣀⣀⠀⠀⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦            ",
    " ⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧           ",
    " ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿           ",
    " ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⠀⠛⠛⠛⠛⠀⠀⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿           ",
    " ⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⡀⠀⠀⢀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿           ",
    "  ⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟            ",
    "   ⠈⠛⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠛⠁             ",
    "      ⠈⠙⠻⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠟⠋⠁                ",
    "          ⠉⠉⠛⠛⠻⠿⠿⠿⠿⠟⠛⠛⠉⠉                    ",
    "                                                ",
    "        ⣿⣿⣿    ⣿⣿⣿⣿⣿⣿⣿⣿    ⣿⣿⣿                ",
    "       ⣿⣿⣿⣿    ⣿⣿⣿⣿⣿⣿⣿⣿    ⣿⣿⣿⣿               ",
    "      ⣿⣿⣿⣿⣿    ⣿⣿⣿⣿⣿⣿⣿⣿    ⣿⣿⣿⣿⣿              ",
    "     ⣿⣿⣿⣿⣿⣿    ⣿⣿⣿⣿⣿⣿⣿⣿    ⣿⣿⣿⣿⣿⣿             ",
    "    ⣿⣿⣿⣿⣿⣿⣿                    ⣿⣿⣿⣿⣿⣿⣿            ",
    "                                                ",
];

/// Number of rows in header art
pub const HEADER_ROWS: usize = 24;

/// Characters per row (including leading/trailing space)
pub const HEADER_COLS: usize = 48;

/// Large block text for "HACHI" title
pub const HACHI_BIG_TEXT: &[&str] = &[
    "██   ██  █████   ██████  ██   ██  ██",
    "██   ██ ██   ██ ██       ██   ██  ██",
    "███████ ███████ ██       ███████  ██",
    "██   ██ ██   ██ ██       ██   ██  ██",
    "██   ██ ██   ██  ██████  ██   ██  ██",
];

/// Bee-themed colors - golden yellow for body, dark for stripes
/// Colors are applied based on character position
const BEE_GOLD: (u8, u8, u8) = (255, 200, 50);      // Golden yellow body
const BEE_AMBER: (u8, u8, u8) = (255, 180, 30);     // Amber highlights
const BEE_HONEY: (u8, u8, u8) = (255, 220, 100);    // Light honey
const BEE_DARK: (u8, u8, u8) = (40, 35, 30);        // Dark stripes
const BEE_BLACK: (u8, u8, u8) = (20, 18, 15);       // Deep black
const BEE_WHITE: (u8, u8, u8) = (255, 255, 255);    // Eyes/highlights
const WING_CYAN: (u8, u8, u8) = (150, 220, 255);    // Translucent wings
const WING_BLUE: (u8, u8, u8) = (100, 180, 230);    // Wing edges

/// Get color for header art at given index
/// Uses a pattern-based approach for the bee design
pub fn header_color(idx: usize) -> Color {
    let row = idx / HEADER_COLS;
    let col = idx % HEADER_COLS;

    let (r, g, b) = get_bee_color(row, col);
    Color::Rgb(r, g, b)
}

/// Determine color based on position in the bee design
fn get_bee_color(row: usize, col: usize) -> (u8, u8, u8) {
    // Rows 0-2: Empty (black background)
    if row < 3 {
        return BEE_BLACK;
    }

    // Rows 3-16: Main bee body
    if row >= 3 && row <= 16 {
        // Center of the design (bee body) - columns roughly 10-36
        if col >= 10 && col <= 36 {
            // Create stripe pattern - alternating gold and dark
            let body_row = row - 3;

            // Eyes area (rows 9-11, specific columns)
            if row >= 9 && row <= 11 {
                // Left eye region
                if col >= 16 && col <= 20 {
                    return BEE_WHITE;
                }
                // Right eye region
                if col >= 26 && col <= 30 {
                    return BEE_WHITE;
                }
            }

            // Stripe pattern (alternating every 2-3 rows)
            match body_row % 5 {
                0 | 1 => BEE_GOLD,
                2 => BEE_DARK,
                3 | 4 => BEE_AMBER,
                _ => BEE_HONEY,
            }
        } else {
            // Background
            BEE_BLACK
        }
    }
    // Rows 17: Gap
    else if row == 17 {
        BEE_BLACK
    }
    // Rows 18-22: Wings
    else if row >= 18 && row <= 22 {
        // Left wing (columns roughly 7-17)
        if col >= 7 && col <= 17 {
            if col < 10 {
                WING_BLUE
            } else {
                WING_CYAN
            }
        }
        // Right wing (columns roughly 29-39)
        else if col >= 29 && col <= 39 {
            if col > 36 {
                WING_BLUE
            } else {
                WING_CYAN
            }
        }
        // Center/legs area
        else if col >= 18 && col <= 28 {
            BEE_DARK
        }
        else {
            BEE_BLACK
        }
    }
    else {
        BEE_BLACK
    }
}

// Legacy compatibility - the old 1256-entry color table
// Now dynamically generated via header_color() function
pub const HEADER_COLORS: [(u8, u8, u8); 1256] = generate_colors();

const fn generate_colors() -> [(u8, u8, u8); 1256] {
    let mut colors = [(0u8, 0u8, 0u8); 1256];
    let mut i = 0;
    while i < 1256 {
        let row = i / HEADER_COLS;
        let col = i % HEADER_COLS;
        colors[i] = get_bee_color_const(row, col);
        i += 1;
    }
    colors
}

/// Const-compatible color selection for compile-time array generation
const fn get_bee_color_const(row: usize, col: usize) -> (u8, u8, u8) {
    // Background for most
    if row < 3 || row > 22 {
        return (20, 18, 15);
    }

    // Main bee body area
    if row >= 3 && row <= 16 && col >= 10 && col <= 36 {
        // Eyes
        if row >= 9 && row <= 11 {
            if (col >= 16 && col <= 20) || (col >= 26 && col <= 30) {
                return (255, 255, 255);
            }
        }
        // Stripes
        let body_row = row - 3;
        let stripe_phase = body_row % 5;
        if stripe_phase == 0 || stripe_phase == 1 {
            (255, 200, 50) // Gold
        } else if stripe_phase == 2 {
            (40, 35, 30) // Dark
        } else {
            (255, 180, 30) // Amber
        }
    }
    // Wings
    else if row >= 18 && row <= 22 {
        if (col >= 7 && col <= 17) || (col >= 29 && col <= 39) {
            (150, 220, 255) // Wing cyan
        } else if col >= 18 && col <= 28 {
            (40, 35, 30) // Legs/center
        } else {
            (20, 18, 15)
        }
    }
    else {
        (20, 18, 15) // Background
    }
}
