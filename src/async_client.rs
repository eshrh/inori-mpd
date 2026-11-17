use crate::convert::FromIter;
use crate::error::{Error, ProtoError, Result};
use crate::proto::{encode, structs, ToArguments};
use crate::reply::Reply;
use crate::Version;
use std::io;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufStream};

#[derive(Debug)]
pub struct AsyncClient<S> {
    socket: BufStream<S>,
    dirty: bool,
    pub version: Version,
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncClient<S> {
    pub async fn new(stream: S) -> Result<Self> {
        let mut socket = BufStream::new(stream);
        let banner = line(&mut socket).await?.ok_or(ProtoError::BadBanner)?;
        let version = banner.strip_prefix("OK MPD ").ok_or(ProtoError::BadBanner)?.trim().parse()?;
        Ok(Self { socket, dirty: false, version })
    }

    pub async fn run<I: ToArguments>(&mut self, command: &str, arguments: I) -> Result<Response> {
        if self.dirty {
            return Err(io::Error::new(io::ErrorKind::BrokenPipe, "previous MPD transaction did not finish").into());
        }
        let command = encode(command, arguments)?;
        self.dirty = true;
        self.socket.write_all(&command).await?;
        self.socket.flush().await?;
        let mut pairs = Vec::new();
        loop {
            let value = match line(&mut self.socket).await? {
                Some(value) => value,
                None => {
                    pairs.push((String::new(), String::new()));
                    continue;
                }
            };
            match value.parse::<Reply>()? {
                Reply::Pair(key, value) => pairs.push((key, value)),
                Reply::Ok if value == "OK" => {
                    self.dirty = false;
                    return Ok(Response(pairs));
                }
                Reply::Ok => return Err(ProtoError::NotOk.into()),
                Reply::Ack(error) => {
                    self.dirty = false;
                    return Err(Error::Server(error));
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Response(Vec<(String, String)>);

impl Response {
    pub fn one<T: FromIter>(self) -> Result<T> {
        T::from_iter(self.0.into_iter().map(Ok))
    }

    pub fn structs<T: FromIter>(self, key: &str) -> Result<Vec<T>> {
        structs(self.0.into_iter().map(Ok), key)
    }

    pub fn list(self, key: &str) -> Vec<String> {
        self.0.into_iter().filter_map(|(field, value)| if field == key { Some(value) } else { None }).collect()
    }

    pub fn pairs(self) -> Vec<(String, String)> {
        self.0
    }
}

async fn line<S: AsyncRead + AsyncWrite + Unpin>(socket: &mut BufStream<S>) -> Result<Option<String>> {
    let mut bytes = Vec::new();
    socket.read_until(b'\n', &mut bytes).await?;
    if !bytes.ends_with(b"\n") {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "MPD response ended before OK").into());
    }
    bytes.pop();
    if bytes.ends_with(b"\r") {
        bytes.pop();
    }
    Ok(String::from_utf8(bytes).ok())
}
