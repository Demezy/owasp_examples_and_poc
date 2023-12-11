use std::io::Cursor;

use rocket::Response;

pub struct BoxedError<'a> {
    pub inner: Box<dyn std::error::Error + 'a>,
}

impl<'a, E> From<E> for BoxedError<'a>
where
    E: std::error::Error + 'a,
{
    fn from(err: E) -> Self {
        return BoxedError {
            inner: Box::new(err),
        };
    }
}

#[rocket::async_trait]
impl<'r> rocket::response::Responder<'r, 'static> for BoxedError<'r> {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let error = self.inner.to_string();

        Response::build()
            .header(rocket::http::ContentType::Plain)
            .sized_body(error.len(), Cursor::new(error))
            .ok()
    }
}
