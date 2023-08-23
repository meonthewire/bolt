use log::LevelFilter;
use env_logger::Builder;

pub fn setup_logging() {
    let mut builder = Builder::new();
    builder.filter(None, LevelFilter::Debug).init();
}