use bincode::{self, deserialize, serialize};
use crate::job::JobTrigger;
use crate::path;
use crate::Context;
use std::error;
use std::fmt;
use std::fs::{remove_file, DirBuilder};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

const SOCKET_FILE_NAME: &str = "toby-workerd.sock";

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Bincode(bincode::Error),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum IpcMessage {
    Job {
        project: String,
        trigger: JobTrigger,
    },
}

#[derive(Debug)]
pub struct IpcServer {
    inner: UnixListener,
    path: PathBuf,
}

#[derive(Debug)]
pub struct IpcClient {
    inner: UnixStream,
}

fn socket_path(context: &Context) -> io::Result<PathBuf> {
    let path = context.runtime_path();

    if !path.exists() {
        DirBuilder::new().recursive(true).create(path)?;
    }

    Ok(path!(path, SOCKET_FILE_NAME))
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(ref err) => write!(f, "{}", err),
            Error::Bincode(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            Error::Io(ref err) => Some(err),
            Error::Bincode(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Self {
        Error::Bincode(err)
    }
}

impl IpcServer {
    pub fn bind(context: &Context) -> io::Result<Self> {
        let path = socket_path(context)?;

        if path.exists() {
            remove_file(&path)?;
        }

        let inner = UnixListener::bind(&path)?;

        Ok(Self { path, inner })
    }

    pub fn receive(&mut self) -> Result<IpcMessage, Error> {
        let (socket, _) = self.inner.accept()?;
        let mut reader = BufReader::new(socket);

        let mut buf = Vec::new();

        reader.read_to_end(&mut buf)?;

        Ok(deserialize(&buf)?)
    }
}

impl Drop for IpcServer {
    fn drop(&mut self) {
        let _ = remove_file(&self.path);
    }
}

impl IpcClient {
    pub fn connect(context: &Context) -> io::Result<Self> {
        let path = socket_path(context)?;

        Ok(Self {
            inner: UnixStream::connect(path)?,
        })
    }

    pub fn send(&mut self, value: &IpcMessage) -> Result<(), Error> {
        let mut writer = BufWriter::new(&mut self.inner);
        let encoded = serialize(value)?;

        writer.write_all(&encoded)?;

        Ok(())
    }
}
