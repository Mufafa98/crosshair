use gtk4::cairo::{Context, Region};
use gtk4::gdk::Display;
use gtk4::{Application, ApplicationWindow, DrawingArea};
use gtk4::{CssProvider, prelude::*};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

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

struct Point {
    x: f64,
    y: f64,
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
    cr.set_source_rgba(color.r, color.g, color.b, color.a); // R, G, B, Alpha
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

fn main() {
    let app = Application::builder()
        .application_id("com.fafa_cross")
        .build();

    app.connect_activate(|app| {
        let provider = CssProvider::new();
        provider.load_from_data("window { background-color: rgba(0, 0, 0, 0.0); }");

        if let Some(display) = Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        let window = ApplicationWindow::new(app);

        window.init_layer_shell();
        window.set_layer(Layer::Overlay);

        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Bottom, true);
        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Right, true);

        window.connect_realize(|window| {
            if let Some(surface) = window.surface() {
                let empty_region = Region::create();
                surface.set_input_region(Some(&empty_region));
            }
        });

        window.present();

        let drawing_area = DrawingArea::new();
        drawing_area.set_draw_func(|_, cr, width, height| {
            let crosshair_config = CrosshairConfig::default();

            let cx = width as f64 / 2.0;
            let cy = height as f64 / 2.0;
            let color = Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            };
            let center = Point { x: cx, y: cy };

            draw_line(cr, &color, &center, &Direction::North, &crosshair_config);
            draw_line(cr, &color, &center, &Direction::East, &crosshair_config);
            draw_line(cr, &color, &center, &Direction::South, &crosshair_config);
            draw_line(cr, &color, &center, &Direction::West, &crosshair_config);
        });

        window.set_child(Some(&drawing_area));
    });

    app.run();
}
