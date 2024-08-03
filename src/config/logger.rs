//logger.rs

use colog;
use std::sync::OnceLock;

static INIT: OnceLock<()> = OnceLock::new();

pub fn init_logging() {
    INIT.get_or_init(|| {
        let mut clog = colog::default_builder();
        clog.filter(None, log::LevelFilter::Debug);
        clog.init();
    });
}

pub fn configure() {
    init_logging();
    // Additional configuration can be added here
}
