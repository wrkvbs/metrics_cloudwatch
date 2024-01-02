use std::{error, fmt};

use metrics::Recorder;

#[derive(Debug)]
pub enum Error {
    BuilderIncomplete(String),
    SetRecorder(metrics::SetRecorderError<Box<dyn Recorder>>),
    Collector,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::BuilderIncomplete(_) => None,
            Self::SetRecorder(src) => Some(src),
            Self::Collector => None,
        }
    }
}
