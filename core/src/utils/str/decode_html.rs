pub trait DecodeHtml {
    fn decode_html(&self) -> String;
}

impl<S: ?Sized + AsRef<str>> DecodeHtml for S {
    fn decode_html(&self) -> String {
        return html_escape::decode_html_entities(self).into();
    }
}
