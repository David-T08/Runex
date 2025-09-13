use gtk::Entry;
use gtk::prelude::*;

use crate::config::Configuration;

pub fn build(config: &Configuration) {
    let entry = Entry::builder()
        .placeholder_text("Type to filter")
        .css_name("runex-query")
        .hexpand(true)
        .height_request(config.query_height)
        .build();
}
