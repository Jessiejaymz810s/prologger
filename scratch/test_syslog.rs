fn main() {
    let formatter = syslog::Formatter3164 {
        facility: syslog::Facility::LOG_USER,
        hostname: None,
        process: "prologger".into(),
        pid: 0,
    };
    if let Ok(mut logger) = syslog::unix(formatter) {
        let _ = logger.err("test error");
    }
}
