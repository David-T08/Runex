use gtk::{Overlay, Align};
use gtk::prelude::*;

pub fn build() -> Overlay {
    let overlay = gtk::Overlay::new();
    overlay.set_hexpand(true);
    overlay.set_halign(Align::Fill);

    overlay.set_vexpand(true);
    overlay.set_valign(Align::Fill);

    overlay
}
