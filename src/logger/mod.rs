use log::{Level, Metadata, Record};

pub static LOGGER: SimpleLogger = SimpleLogger;

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprint!("{}", record.args());
        }
    }

    fn flush(&self) {}
}
