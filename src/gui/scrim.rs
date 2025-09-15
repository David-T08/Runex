use gtk::prelude::*;
use gtk::{Align, Box};

pub fn build() -> Box {
    let scrim = Box::new(gtk::Orientation::Vertical, 0);
    scrim.add_css_class("overlay-scrim");

    scrim.set_valign(Align::Fill);
    scrim.set_vexpand(true);

    scrim.set_halign(Align::Fill);
    scrim.set_hexpand(true);

    scrim
}
