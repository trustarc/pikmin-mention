pub struct Frontmost {
    pub name: String,
    pub bundle_id: String,
    pub pid: i32,
}

pub struct Clipboard {
    pub text: String,
    pub html: Option<String>,
}
