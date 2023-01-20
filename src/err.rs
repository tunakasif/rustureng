use reqwest::{Error, Response};

#[derive(Debug)]
pub enum RusTurengError {
    Reqwest(Error),
    ResponseNotOk(Response),
}

impl From<Error> for RusTurengError {
    fn from(err: Error) -> Self {
        RusTurengError::Reqwest(err)
    }
}

impl From<Response> for RusTurengError {
    fn from(resp: Response) -> Self {
        RusTurengError::ResponseNotOk(resp)
    }
}
