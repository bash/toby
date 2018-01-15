use super::worker::Job;
use super::ipc::Sender;
use rocket_contrib::Json;
use rocket::{self, State};
use rocket::http::Status;
use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender};

#[derive(Serialize, Deserialize)]
struct BuildResponse {
    queued: bool,
}

#[get("/v1/build/<project>/<token>")]
fn build(
    tx: State<SyncSender<Job>>,
    project: String,
    token: String,
) -> Result<Json<BuildResponse>, Status> {
    // TODO: validate token and project

    match tx.send(Job::new(project)) {
        Ok(_) => Ok(Json(BuildResponse { queued: true })),
        Err(_) => Err(Status::InternalServerError),
    }

    // Err(Status::Forbidden)
}

pub fn start_server() {
    let sender: Sender<Job> = Sender::connect().expect("unable to connect to ipc server");
    let (tx, rx) = sync_channel::<Job>(1024);

    // TODO: implement proper error handling
    thread::spawn(move || loop {
        let job = rx.recv().unwrap();

        sender.send(job).unwrap();
    });

    rocket::ignite()
        .manage(tx)
        .mount("/", routes![build])
        .launch();
}
