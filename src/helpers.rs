pub fn current_time_string() -> String {
    time::OffsetDateTime::now_local()
        .format("%F %r")
        .to_string()
}
