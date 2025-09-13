use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, CssProvider, Orientation, gdk, gio};

use gdk::Display;
use gio::ActionEntry;
use glib::clone;

use anyhow::{Context, Result};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

use crate::CliArgs;
use crate::config::{Configuration, config_home_path};

mod search;

pub fn build_application(
    app_id: &str,
    cli_args: CliArgs,
    app_config: Configuration,
) -> Result<gtk::Application> {
    let app = Application::builder().application_id(app_id).build();

    let style_path = cli_args.style.clone();
    app.connect_startup(clone!(
        #[strong]
        style_path,
        move |_| load_css(style_path.clone())
    ));

    app.connect_activate(clone!(
        #[strong]
        app_config,

        move |app| {
            build_ui(app, &app_config)
        }
    ));

    app.set_accels_for_action("win.close", &["Escape"]);

    Ok(app)
}

fn build_ui(app: &Application, config: &Configuration) {
    let container = gtk::Box::builder()
        .opacity(1.0)
        .orientation(Orientation::Vertical)
        .width_request(config.width)
        .vexpand(true)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .decorated(false)
        .resizable(false)
        .child(&container)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);

    let action_close = ActionEntry::builder("close")
        .activate(|window: &ApplicationWindow, _, _| {
            window.close();
        })
        .build();

    window.add_action_entries([action_close]);

    window.present();
}

fn load_css(path: Option<PathBuf>) {
    let provider = CssProvider::new();
    let path = path.unwrap_or(config_home_path().join("runex").join("style.css"));

    provider.load_from_path(&path);

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
