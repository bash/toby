use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use ipc_channel;
use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};

const IPC_NAME_FILE: &'static str = "/tmp/toby-ipc-server-name";

#[derive(Debug)]
pub enum IpcError {
    FsError,
    ConnectionError,
}

pub struct Server<T>
where
    T: Serialize,
{
    #[allow(dead_code)] name_file: NameFile,
    server: IpcOneShotServer<T>,
}

pub struct Receiver<T>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    receiver: IpcReceiver<T>,
    first_msg: Option<T>,
}

pub struct Sender<T>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    sender: IpcSender<T>,
}

struct NameFile;

impl<T> Server<T>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    pub fn new() -> Result<Self, IpcError> {
        let (server, name) = match IpcOneShotServer::new() {
            Ok(val) => val,
            Err(..) => return Err(IpcError::FsError),
        };

        let name_file = match NameFile::create(name) {
            Ok(file) => file,
            Err(..) => return Err(IpcError::FsError),
        };

        Ok(Server { server, name_file })
    }

    pub fn connect(self) -> Result<Receiver<T>, IpcError> {
        let Self { server, .. } = self;
        let (receiver, first_msg) = match server.accept() {
            Ok(val) => val,
            Err(..) => return Err(IpcError::ConnectionError),
        };

        Ok(Receiver::new(receiver, first_msg))
    }
}

impl<T> Receiver<T>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    fn new(receiver: IpcReceiver<T>, first_msg: T) -> Self {
        Receiver {
            receiver,
            first_msg: Some(first_msg),
        }
    }
}

impl<T> Iterator for Receiver<T>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    type Item = Result<T, IpcError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.first_msg.take() {
            Some(msg) => return Some(Ok(msg)),
            None => {}
        };

        match self.receiver.recv() {
            Ok(msg) => Some(Ok(msg)),
            Err(..) => Some(Err(IpcError::ConnectionError)),
        }
    }
}

impl<T> Sender<T>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    pub fn connect() -> Result<Self, IpcError> {
        let name = {
            let mut file = match File::open(IPC_NAME_FILE) {
                Ok(file) => file,
                Err(..) => return Err(IpcError::FsError),
            };

            let mut name = String::new();

            match file.read_to_string(&mut name) {
                Ok(..) => {}
                Err(..) => return Err(IpcError::FsError),
            };

            name
        };

        let sender = match IpcSender::connect(name) {
            Ok(sender) => sender,
            Err(..) => return Err(IpcError::ConnectionError),
        };

        Ok(Sender { sender })
    }

    pub fn send(&self, msg: T) -> Result<(), ipc_channel::Error> {
        self.sender.send(msg)
    }
}

impl NameFile {
    fn create(name: String) -> io::Result<Self> {
        let mut file = File::create(IPC_NAME_FILE)?;

        file.write_all(name.as_bytes())?;

        Ok(NameFile)
    }
}

impl Drop for NameFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(IPC_NAME_FILE);
    }
}
