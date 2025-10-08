pub fn t(key: &str) -> String {
    // stub très simple
    match key {
        "hello" => "bonjour".to_string(),
        _ => key.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_translates() {
        assert_eq!(t("hello"), "bonjour");
    }
}
