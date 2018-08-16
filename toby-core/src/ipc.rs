use bincode::{self, deserialize, serialize};
use crate::job::JobTrigger;
use crate::path;
use crate::Context;
use std::error;
use std::ffi::CString;
use std::fmt;
use std::fs::{remove_file, DirBuilder};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use users::mock::{gid_t, uid_t};

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
pub struct IpcServerBuilder<'a> {
    context: &'a Context,
    owner: Option<(uid_t, gid_t)>,
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

fn chown(path: &Path, uid: uid_t, gid: gid_t) -> io::Result<()> {
    let s = CString::new(path.as_os_str().as_bytes()).unwrap();
    let ret = unsafe { libc::chown(s.as_ptr(), uid, gid) };

    if ret == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
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

impl<'a> IpcServerBuilder<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            owner: None,
        }
    }

    pub fn owner(mut self, owner: (uid_t, gid_t)) -> Self {
        self.owner = Some(owner);
        self
    }

    pub fn bind(self) -> io::Result<IpcServer> {
        let path = socket_path(self.context)?;

        if path.exists() {
            remove_file(&path)?;
        }

        let inner = UnixListener::bind(&path)?;

        if let Some((uid, gid)) = self.owner {
            chown(&path, uid, gid)?;
        }

        Ok(IpcServer { path, inner })
    }
}

impl IpcServer {
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
