use rustyline::error::ReadlineError;

#[derive(Debug)]
pub enum CmdLineError {
    Quit,
    ReadlineError(ReadlineError),
    AddHistoryError(ReadlineError),
    ParseProtoError(ferox::proto::errors::Error),
    PostcardError(postcard::Error),
    SerialPortError(std::io::Error),
}

pub type CmdResult<T> = core::result::Result<T, CmdLineError>;
