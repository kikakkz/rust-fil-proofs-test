#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;
#[macro_use]
extern crate lazy_static;

use slog::Drain;
use slog::FnValue;
use slog::Level;
use slog::LevelFilter;
use slog::Logger;
use std::env;

lazy_static! {
    static ref ROOT_LOGGER: Logger =
        make_root_logger("FIL_PROOFS_LOG_JSON", "FIL_PROOFS_MIN_LOG_LEVEL");
}

pub fn make_root_logger(use_json_env_name: &str, min_log_level_env_name: &str) -> Logger {
    let min_log_level = match env::var(min_log_level_env_name) {
        Ok(val) => match val.parse::<u64>() {
            Ok(parsed) => match Level::from_usize(parsed as usize) {
                Some(level) => level,
                None => Level::Info,
            },
            _ => Level::Info,
        },
        _ => Level::Info,
    };

    let logging_config = o!("place" => FnValue(move |info| {
        format!("{}:{} {}",
                info.file(),
                info.line(),
                info.module(),
                )
    }));

    let use_json_logger = env::var(use_json_env_name)
        .map(|x| x == "true")
        .unwrap_or(false);

    if use_json_logger {
        let formatted = slog_json::Json::new(std::io::stdout())
            .add_default_keys()
            .build()
            .fuse();
        let mutexed = std::sync::Mutex::new(formatted).fuse();
        let filtered = LevelFilter::new(mutexed, min_log_level).map(slog::Fuse);
        Logger::root(filtered, logging_config)
    } else {
        let term_decorator = slog_term::TermDecorator::new().build();
        let formatted = slog_term::FullFormat::new(term_decorator).build().fuse();
        let mutexed = std::sync::Mutex::new(formatted).fuse();
        let filtered = LevelFilter::new(mutexed, min_log_level).map(slog::Fuse);
        Logger::root(filtered, logging_config)
    }
}

pub fn make_logger(root_name: &'static str) -> Logger {
    ROOT_LOGGER.new(o!("root" => root_name))
}
