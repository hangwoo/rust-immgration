#[derive(PartialOrd, PartialEq, Debug)]
pub struct Logger {
    verbose_level: LogLevel,
}

impl Logger {
    pub fn new(verbose_level: LogLevel) -> Logger {
        Logger {
            verbose_level
        }
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Error,
}

#[cfg(test)]
mod tests {
    use crate::logger::*;

    #[test]
    fn get_logger() {
        assert_eq!(Logger::new(LogLevel::Info), Logger { verbose_level: LogLevel::Info });
        assert_eq!(Logger::new(LogLevel::Error), Logger { verbose_level: LogLevel::Error });
        assert_ne!(Logger::new(LogLevel::Error), Logger { verbose_level: LogLevel::Info });
    }
}