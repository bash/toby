use bincode::{self, deserialize, serialize};
use crate::job::JobTrigger;
use crate::Context;
use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;
use std::fs::{remove_file, DirBuilder};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::marker::PhantomData;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

const SOCKET_FILE_NAME: &str = "toby-workerd.sock";

pub type IpcServer = IpcServerImpl<IpcMessage>;
pub type IpcClient = IpcClientImpl<IpcMessage>;

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
pub struct IpcServerImpl<T> {
    inner: UnixListener,
    path: PathBuf,
    phantom: PhantomData<T>,
}

#[derive(Debug)]
pub struct IpcClientImpl<T>
where
    T: Serialize,
{
    inner: UnixStream,
    phantom: PhantomData<T>,
}

fn socket_path(context: &Context) -> io::Result<PathBuf> {
    let mut path = PathBuf::from(context.runtime_path());

    if !path.exists() {
        DirBuilder::new().recursive(true).create(&path)?;
    }

    path.push(SOCKET_FILE_NAME);

    Ok(path)
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

impl<T> IpcServerImpl<T>
where
    T: for<'de> Deserialize<'de>,
{
    pub fn bind(context: &Context) -> io::Result<Self> {
        let path = socket_path(context)?;

        if path.exists() {
            remove_file(&path)?;
        }

        let inner = UnixListener::bind(&path)?;

        Ok(Self {
            path,
            inner,
            phantom: PhantomData,
        })
    }

    pub fn receive(&mut self) -> Result<T, Error> {
        let (socket, _) = self.inner.accept()?;
        let mut reader = BufReader::new(socket);

        let mut buf = Vec::new();

        reader.read_to_end(&mut buf)?;

        Ok(deserialize(&buf)?)
    }
}

impl<T> Drop for IpcServerImpl<T> {
    fn drop(&mut self) {
        let _ = remove_file(&self.path);
    }
}

impl<T> IpcClientImpl<T>
where
    T: Serialize,
{
    pub fn connect(context: &Context) -> io::Result<Self> {
        let path = socket_path(context)?;

        Ok(Self {
            inner: UnixStream::connect(path)?,
            phantom: PhantomData,
        })
    }

    pub fn send(&mut self, value: &T) -> Result<(), Error> {
        let mut writer = BufWriter::new(&mut self.inner);
        let encoded = serialize(value)?;

        writer.write_all(&encoded)?;

        Ok(())
    }
}
