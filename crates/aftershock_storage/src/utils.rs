pub fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System time error! Do we have a time machine?")
        .as_secs() as i64
}
