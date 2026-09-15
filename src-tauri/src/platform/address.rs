pub fn looks_like_url(value: &str) -> bool {
    let trimmed = value.trim();

    if trimmed.is_empty() || trimmed.contains(' ') {
        return false;
    }

    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        return true;
    }

    if !trimmed.contains('.') {
        return false;
    }

    let authority = trimmed.split(['/', '?', '#']).next().unwrap_or(trimmed);

    !authority.contains('@')
}

#[cfg(test)]
mod tests {
    use super::looks_like_url;

    #[test]
    fn accepts_an_address_with_a_scheme() {
        assert!(looks_like_url("https://github.com/trustarc/pikmin-mention"));
        assert!(looks_like_url("http://localhost:1420"));
    }

    #[test]
    fn accepts_an_address_the_browser_shows_without_its_scheme() {
        assert!(looks_like_url("github.com"));
        assert!(looks_like_url(
            "mailchi.mp/6f6cd7bb06fd/primeui-monthly?e=50f82d4ac8"
        ));
    }

    #[test]
    fn keeps_an_at_sign_in_the_path_or_the_query() {
        assert!(looks_like_url("https://example.com/@alice"));
        assert!(looks_like_url(
            "https://example.com/?email=alice@example.com"
        ));
        assert!(looks_like_url("example.com/@alice"));
        assert!(looks_like_url("example.com/?email=alice@example.com"));
    }

    #[test]
    fn rejects_an_email_address() {
        assert!(!looks_like_url("alice@example.com"));
    }

    #[test]
    fn rejects_text_that_is_not_an_address() {
        assert!(!looks_like_url(""));
        assert!(!looks_like_url("   "));
        assert!(!looks_like_url("Search with Google or enter address"));
        assert!(!looks_like_url("pikmin"));
    }
}
