const START_MARKER: &str = "<!--StartFragment-->";
const END_MARKER: &str = "<!--EndFragment-->";

fn header_offset(text: &str, key: &str) -> Option<usize> {
    text.lines()
        .take_while(|line| !line.trim_start().starts_with('<'))
        .find_map(|line| line.strip_prefix(key))
        .and_then(|value| value.trim().parse().ok())
}

fn between_markers(text: &str) -> Option<String> {
    let start = text.find(START_MARKER)? + START_MARKER.len();
    let end = text.find(END_MARKER)?;
    (start <= end).then(|| text[start..end].to_string())
}

fn between_offsets(raw: &[u8], text: &str) -> Option<String> {
    let header_end = header_offset(text, "StartHTML:")?;
    let start = header_offset(text, "StartFragment:")?;
    let end = header_offset(text, "EndFragment:")?;

    (start >= header_end && start < end && end <= raw.len())
        .then(|| String::from_utf8_lossy(&raw[start..end]).into_owned())
}

fn whole_document(text: &str) -> Option<String> {
    let at = text.find("<html").or_else(|| text.find("<HTML"))?;
    Some(text[at..].to_string())
}

pub fn html_fragment(raw: &[u8]) -> Option<String> {
    let text = String::from_utf8_lossy(raw);

    let fragment = between_markers(&text)
        .or_else(|| between_offsets(raw, &text))
        .or_else(|| whole_document(&text))?;

    let trimmed = fragment
        .replace(START_MARKER, "")
        .replace(END_MARKER, "")
        .trim()
        .to_string();

    (!trimmed.is_empty()).then_some(trimmed)
}

#[cfg(test)]
mod tests {
    use super::html_fragment;

    const MENTION: &str = r#"<a href="https://trustarc.slack.com/team/U1">@Pikmin</a>"#;

    fn header(a: usize, b: usize, c: usize, d: usize) -> String {
        format!(
            "Version:0.9\r\nStartHTML:{a:010}\r\nEndHTML:{b:010}\r\nStartFragment:{c:010}\r\nEndFragment:{d:010}\r\n"
        )
    }

    fn cf_html(before: &str, fragment: &str, after: &str) -> Vec<u8> {
        let start_html = header(0, 0, 0, 0).len();
        let start_fragment = start_html + before.len();
        let end_fragment = start_fragment + fragment.len();
        let end_html = end_fragment + after.len();

        let mut raw = header(start_html, end_html, start_fragment, end_fragment).into_bytes();
        raw.extend_from_slice(before.as_bytes());
        raw.extend_from_slice(fragment.as_bytes());
        raw.extend_from_slice(after.as_bytes());
        raw
    }

    #[test]
    fn keeps_only_what_is_between_the_markers() {
        let raw = cf_html(
            "<html>\r\n<body>\r\n<!--StartFragment-->",
            MENTION,
            "<!--EndFragment-->\r\n</body>\r\n</html>",
        );

        assert_eq!(html_fragment(&raw).as_deref(), Some(MENTION));
    }

    #[test]
    fn falls_back_to_the_header_offsets() {
        let raw = cf_html("<html><body>", MENTION, "</body></html>");

        assert_eq!(html_fragment(&raw).as_deref(), Some(MENTION));
    }

    #[test]
    fn falls_back_to_the_document_without_a_header() {
        let raw = b"<html><body>plain</body></html>";

        assert_eq!(
            html_fragment(raw).as_deref(),
            Some("<html><body>plain</body></html>")
        );
    }

    #[test]
    fn rejects_an_empty_fragment() {
        let raw = cf_html(
            "<html><body><!--StartFragment-->",
            "  ",
            "<!--EndFragment--></body></html>",
        );

        assert_eq!(html_fragment(&raw), None);
    }

    #[test]
    fn never_returns_the_header_itself() {
        let body = format!("<html><body>{MENTION}</body></html>");
        let start_html = header(0, 0, 0, 0).len();
        let mut raw = header(start_html, start_html + body.len(), 0, start_html).into_bytes();
        raw.extend_from_slice(body.as_bytes());

        let fragment = html_fragment(&raw).expect("a fragment");

        assert!(!fragment.contains("Version:0.9"));
        assert!(fragment.starts_with("<html"));
    }
}
