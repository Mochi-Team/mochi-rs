use mochi_imports::http::{HTTPRequest, HTTPMethod};

pub struct VidStreaming {}

impl VidStreaming {
    pub fn extract<T: AsRef<str>>(video_url: T) {
        let value = video_url.as_ref();
        let html = HTTPRequest::new("", HTTPMethod::Get);
    }
}