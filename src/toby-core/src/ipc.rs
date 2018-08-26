use bincode::{self, deserialize, serialize};
use crate::identity::Identity;
use crate::job::JobTrigger;
use crate::path;
use crate::Context;
use futures::{future, Future, IntoFuture, Stream};
use std::error;
use std::ffi::CString;
use std::fmt;
use std::fs::{remove_file, DirBuilder};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use tokio::io::{read_to_end, write_all};
use tokio::net::unix::{UnixListener, UnixStream};
use users::mock::{gid_t, uid_t};

const SOCKET_FILE_NAME: &str = "toby-workerd.sock";

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    TokioIo(tokio::io::Error),
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
pub struct IpcServerBuilder<'a> {
    context: &'a Context,
    owner: Option<(uid_t, gid_t)>,
}

#[derive(Debug)]
pub struct IpcClient {
    inner: UnixStream,
}

fn socket_path(context: &Context) -> std::io::Result<PathBuf> {
    let path = context.runtime_path();

    if !path.exists() {
        DirBuilder::new().recursive(true).create(path)?;
    }

    Ok(path!(path, SOCKET_FILE_NAME))
}

fn chown(path: &Path, uid: uid_t, gid: gid_t) -> std::io::Result<()> {
    let s = CString::new(path.as_os_str().as_bytes()).unwrap();
    let ret = unsafe { libc::chown(s.as_ptr(), uid, gid) };

    if ret == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(ref err) => write!(f, "{}", err),
            Error::Bincode(ref err) => write!(f, "{}", err),
            Error::TokioIo(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            Error::Io(ref err) => Some(err),
            Error::Bincode(ref err) => Some(err),
            Error::TokioIo(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Self {
        Error::Bincode(err)
    }
}

impl<'a> IpcServerBuilder<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            owner: None,
        }
    }

    pub fn owner(mut self, owner: &Identity) -> Self {
        self.owner = Some((owner.uid(), owner.gid()));
        self
    }

    pub fn bind(self) -> impl Future<Item = IpcServer, Error = Error> {
        let Self { context, owner } = self;

        let path_future = socket_path(context).map_err(Into::into).into_future();

        let path_remove_future = path_future.and_then(|path| {
            if path.exists() {
                return remove_file(&path)
                    .map_err(Into::into)
                    .map(|_| path)
                    .into_future();
            }

            future::ok(path)
        });

        let bind_future = path_remove_future.and_then(|path| {
            UnixListener::bind(&path)
                .map_err(Into::into)
                .map(|inner| (path, inner))
        });

        let chown_future = bind_future.and_then(move |(path, inner)| {
            if let Some((uid, gid)) = owner {
                return chown(&path, uid, gid)
                    .map_err(Into::into)
                    .map(|_| (path, inner))
                    .into_future();
            }

            future::ok((path, inner))
        });

        chown_future.map(|(path, inner)| IpcServer { path, inner })
    }
}

impl IpcServer {
    pub fn incoming(self) -> impl Stream<Item = IpcMessage, Error = Error> {
        self.inner
            .incoming()
            .map_err(Into::into)
            .and_then(|unix_stream| {
                read_to_end(unix_stream, Vec::new())
                    .map_err(Into::into)
                    .and_then(|(_, buf)| Ok(deserialize(&buf)?))
            })
    }
}

impl IpcClient {
    pub fn connect(context: &Context) -> impl Future<Item = Self, Error = Error> {
        socket_path(context)
            .map_err(Into::into)
            .into_future()
            .and_then(|path| UnixStream::connect(path).map_err(Into::into))
            .map(|inner| Self { inner })
    }

    pub fn send<'a>(self, value: &IpcMessage) -> impl Future<Item = Self, Error = Error> + 'a {
        serialize(value)
            .map_err(Into::into)
            .into_future()
            .and_then(|serialized| {
                write_all(self.inner, serialized)
                    .map(|(inner, _)| Self { inner })
                    .map_err(Into::into)
            })
    }
}
