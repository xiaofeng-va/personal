use super::data::FeroxProto;
use super::data::{FeroxProto::FeroxRequest, FeroxRequestType::FeroxPing};
use super::data::{Ctl200RequestType::Ctl200Version, FeroxProto::Ctl200Request};
use super::errors::Result;
use super::errors::Error;

pub fn parse_proto(line: &str) -> Result<FeroxProto> {
    let mut tokens = line.split_whitespace();
    parse_command(&mut tokens)
}

fn parse_command<'a, I>(tokens: &mut I) -> Result<FeroxProto>
where 
    I: Iterator<Item = &'a str>,
{
    match tokens.next() {
        Some("quit") => Ok(FeroxProto::Quit),
        Some("ferox") => parse_ferox_command(tokens),
        Some("ctl200") => parse_ctl200_command(tokens),
        _ => Err(Error::InvalidCommand),
    }
}

fn parse_ferox_command<'a, I>(tokens: &mut I) -> Result<FeroxProto>
where 
    I: Iterator<Item = &'a str>,
{
    match tokens.next() {
        Some("ping") => Ok(FeroxRequest(FeroxPing)),
        _ => Err(Error::InvalidCommand),
    }
}

fn parse_ctl200_command<'a, I>(tokens: &mut I) -> Result<FeroxProto>
where 
    I: Iterator<Item = &'a str>,
{
    match tokens.next() {
        Some("version") => Ok(Ctl200Request(Ctl200Version)),
        _ => Err(Error::InvalidCommand),
    }
}