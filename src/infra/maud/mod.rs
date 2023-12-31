use std::io::Cursor;

use maud::PreEscaped;
use rocket::{
    http::ContentType,
    response::{self, Responder},
    Request, Response,
};

pub struct MaudTemplate {
    pub(crate) string: PreEscaped<String>,
    pub(crate) headers: Option<Vec<(String, String)>>,
}

impl MaudTemplate {
    fn len(&self) -> usize {
        self.string.0.len()
    }
}

impl From<PreEscaped<String>> for MaudTemplate {
    fn from(s: PreEscaped<String>) -> Self {
        MaudTemplate {
            string: s,
            headers: None,
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for MaudTemplate {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'o> {
        let mut builder = Response::build();
        builder.header(ContentType::HTML);

        let len = self.len();
        let headers = self.headers;
        let string = self.string;

        match headers {
            Some(headers) => {
                for (key, value) in headers {
                    builder.raw_header(key, value);
                }
            }
            _ => (),
        }
        builder.sized_body(len, Cursor::new(string.0)).ok()
    }
}
