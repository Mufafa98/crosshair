use gtk4::cairo::{Context, Region};
use gtk4::gdk::Display;
use gtk4::{Application, ApplicationWindow, DrawingArea};
use gtk4::{CssProvider, prelude::*};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

use crate::crosshair::{Point, draw_crosshair};

mod crosshair;
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
            let cx = width as f64 / 2.0;
            let cy = height as f64 / 2.0;
            let center = Point { x: cx, y: cy };

            draw_crosshair(cr, &center);
        });

        window.set_child(Some(&drawing_area));
    });

    app.run();
}
