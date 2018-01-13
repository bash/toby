use super::worker::Job;
use super::ipc::Sender;

pub fn start_server() {
    let sender = Sender::connect().expect("unable to connect to ipc server");

    sender.send(Job::new("foo")).unwrap();
    sender.send(Job::new("bar")).unwrap();
}
