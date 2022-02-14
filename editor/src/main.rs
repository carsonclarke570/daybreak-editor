use engine_core::{self};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();
    engine_core::test()
}
