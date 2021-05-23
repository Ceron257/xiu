use failure::Fail;
use rtmp::session::errors::SessionError;

use networkio::bytes_errors::BytesWriteError;
use rtmp::amf0::errors::Amf0WriteError;

#[derive(Debug)]
pub struct ServerError {
    pub value: ServerErrorValue,
}

#[derive(Debug, Fail)]
pub enum ServerErrorValue {
    #[fail(display = "server error")]
    Error,
}

pub struct HttpFLvError {
    pub value: HttpFLvErrorValue,
}

#[derive(Debug, Fail)]
pub enum HttpFLvErrorValue {
    #[fail(display = "server error")]
    Error,
    #[fail(display = "session error")]
    SessionError(SessionError),
    #[fail(display = "bytes write error")]
    BytesWriteError(BytesWriteError),
    #[fail(display = "amf write error")]
    AmfWriteError(Amf0WriteError),
}

impl From<SessionError> for HttpFLvError {
    fn from(error: SessionError) -> Self {
        HttpFLvError {
            value: HttpFLvErrorValue::SessionError(error),
        }
    }
}

impl From<BytesWriteError> for HttpFLvError {
    fn from(error: BytesWriteError) -> Self {
        HttpFLvError {
            value: HttpFLvErrorValue::BytesWriteError(error),
        }
    }
}

impl From<Amf0WriteError> for HttpFLvError {
    fn from(error: Amf0WriteError) -> Self {
        HttpFLvError {
            value: HttpFLvErrorValue::Amf0WriteError(error),
        }
    }
}