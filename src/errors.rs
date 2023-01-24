use reqwest::Response;

#[derive(Debug)]
pub enum ReweError {
    ReqwestError(reqwest::Error),
    StatusCodeClientError(Response),
    StatusCodeServerError(Response)
}

impl From<reqwest::Error> for ReweError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}