trait StringProvider {
    fn provide(&self) -> String;
}

impl StringProvider for fn() -> String {
    fn provide(&self) -> String {
        self()
    }
}

impl StringProvider for fn() -> Option<String> {
    fn provide(&self) -> String {
        self().unwrap_or_else(|| "None".to_string())
    }
}

