use gtk::{Entry};

use crate::config::ResolvedConfig;

pub fn build(config: &ResolvedConfig) -> Entry {
    let entry = Entry::builder()
        .placeholder_text("Type to filter")
        .css_classes(["query"])
        .hexpand(true)
        .build();

    entry
}
