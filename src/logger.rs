const USE_VERBOSE_LOGS: bool = true;

pub fn log_verbose(msg: String) {
    if USE_VERBOSE_LOGS {
        println!("{}", msg);
    }
}
