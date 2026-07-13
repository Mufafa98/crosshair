use gtk4::cairo::Context;

struct CrosshairConfig {
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

pub fn draw_crosshair(cr: &Context, center: &Point) {
    let config = CrosshairConfig::default();
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
