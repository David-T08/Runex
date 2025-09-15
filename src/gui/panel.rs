use gtk::{Box, Orientation, Align};

pub fn build() -> Box {
    Box::builder()
        .css_classes(["launcher-panel"])
        .halign(Align::Center)
        .orientation(Orientation::Vertical)
        .build()
}