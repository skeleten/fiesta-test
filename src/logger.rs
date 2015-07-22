use std::sync::Mutex;
use log::*;

pub struct EnvLogger;

impl EnvLogger {
	pub fn init() -> Result<(), SetLoggerError> {
	    set_logger(|max_log_level| {
	        max_log_level.set(LogLevelFilter::Off);
	        Box::new(EnvLogger)
	    })
	}
}

impl Log for EnvLogger {
	#[allow(unused_variables)]
    fn enabled(&self, metadata: &LogMetadata) -> bool {
    	true
    }

    fn log(&self, record: &LogRecord) {
    	println!("[{:12}] '{}'",
    		record.target(),
    		record.args());
    }
}