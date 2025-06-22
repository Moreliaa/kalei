const USE_VERBOSE_LOGS: bool = false;

pub fn log_verbose(msg: String) {
    if USE_VERBOSE_LOGS {
        println!("{}", msg);
    }
}
