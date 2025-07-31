pub fn get() -> Option<String> {
    std::env::var("SHELL").ok()
}
