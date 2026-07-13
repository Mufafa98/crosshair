use gtk4::cairo::Context;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};
use toml;

#[derive(Clone, Deserialize, Serialize)]
pub struct CrosshairConfig {
    gap: f64,
    len: f64,
    width: f64,
}

impl Default for CrosshairConfig {
    fn default() -> Self {
        Self {
            gap: 5.0,
            len: 15.0,
            width: 2.0,
        }
    }
}

impl CrosshairConfig {
    pub fn load_from_file() -> Self {
        if cfg!(target_os = "windows") {
            return Self::default();
        } else if cfg!(target_os = "linux") {
            let home = env::var("HOME").expect("HOME directory not found");
            let config_dir = PathBuf::from(home).join(".config").join("fafa_cross");
            let config_path = config_dir.join("config.toml");

            if !config_path.exists() {
                fs::create_dir_all(&config_dir).unwrap();
                let default_config = Self::default();
                let toml_string = toml::to_string(&default_config).unwrap();
                fs::write(&config_path, toml_string).unwrap();

                return default_config;
            }

            // Read existing file
            return fs::read_to_string(&config_path)
                .ok()
                .and_then(|content| toml::from_str(&content).ok())
                .unwrap_or_default();
        }
        return Self::default();
    }
}

struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn draw_line(
    cr: &Context,
    color: &Color,
    center: &Point,
    direction: &Direction,
    config: &CrosshairConfig,
) {
    cr.set_source_rgba(color.r, color.g, color.b, color.a);
    cr.set_line_width(config.width);

    let x = center.x;
    let y = center.y;

    let len = config.len;
    let gap = config.gap;

    match direction {
        Direction::North => {
            cr.move_to(x, y - gap);
        }
        Direction::East => {
            cr.move_to(x + gap, y);
        }
        Direction::South => {
            cr.move_to(x, y + gap);
        }
        Direction::West => {
            cr.move_to(x - gap, y);
        }
    }

    match direction {
        Direction::North => {
            cr.line_to(x, y - len);
        }
        Direction::East => {
            cr.line_to(x + len, y);
        }
        Direction::South => {
            cr.line_to(x, y + len);
        }
        Direction::West => {
            cr.line_to(x - len, y);
        }
    }

    cr.stroke().unwrap();
}

pub fn draw_crosshair(cr: &Context, center: &Point, config: &CrosshairConfig) {
    let color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    draw_line(cr, &color, &center, &Direction::North, &config);
    draw_line(cr, &color, &center, &Direction::East, &config);
    draw_line(cr, &color, &center, &Direction::South, &config);
    draw_line(cr, &color, &center, &Direction::West, &config);
}
