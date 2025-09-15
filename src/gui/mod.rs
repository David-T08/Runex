use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, CssProvider, gdk, gio};

use gdk::Display;
use gio::ActionEntry;
use glib::clone;

use anyhow::{Context, Result};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

use crate::CliArgs;
use crate::config::{ResolvedConfig, ResolvedScrim, config_home_path};

mod overlay;
mod scrim;
mod search;
mod panel;

pub fn build_application(
    app_id: &str,
    cli_args: CliArgs,
    app_config: ResolvedConfig,
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
        move |app| build_ui(app, &app_config)
    ));

    app.set_accels_for_action("win.close", &["Escape"]);

    Ok(app)
}


fn setup_keybindings(window: &ApplicationWindow) {
    let action_close = ActionEntry::builder("close")
        .activate(|window: &ApplicationWindow, _, _| {
            eprintln!("poressed esc");
            window.close();
        })
        .build();

    // Safety to close automatically
    glib::timeout_add_seconds_local_once(
        30,
        clone!(
            #[weak]
            window,
            move || {
                eprintln!("safety close");
                window.close()
            }
        ),
    );

    window.connect_close_request(|_| {
        eprintln!("close-request fired");
        glib::Propagation::Proceed
    });

    window.add_action_entries([action_close]);
}


fn setup_gls(window: &ApplicationWindow) {
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);

    window.set_exclusive_zone(-1);
    window.set_namespace(Some("runex"));
}


fn apply_scrim(window: &ApplicationWindow, s: &ResolvedScrim) {
    for e in &s.edges {
        window.set_anchor((*e).into(), true);
    }

    window.set_margin(Edge::Left, s.margins.left);
    window.set_margin(Edge::Right, s.margins.right);
    window.set_margin(Edge::Top, s.margins.top);
    window.set_margin(Edge::Bottom, s.margins.bottom);
}


fn build_ui(app: &Application, config: &ResolvedConfig) {
    let overlay = overlay::build();
    let scrim = scrim::build();

    let panel = panel::build();
    let search = search::build(&config);

    panel.append(&search);

    overlay.set_child(Some(&scrim));
    overlay.add_overlay(&panel);

    let window = ApplicationWindow::builder()
        .application(app)
        .decorated(false)
        .resizable(true)
        .child(&overlay)
        .build();

    setup_gls(&window);
    setup_keybindings(&window);
    apply_scrim(&window, &config.scrim);

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
