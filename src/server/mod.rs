use super::worker::Job;
use super::ipc::Sender;
use super::config::{get_config, get_projects, Projects};
use rocket_contrib::Json;
use rocket::{self, State};
use rocket::config::{ConfigBuilder, Environment};
use rocket::http::Status;
use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender};

// TODO: what value should I have here?
// Note to future self: 8 was picked arbitrarily
const CHANNEL_BOUND: usize = 8;

#[derive(Serialize, Deserialize)]
struct BuildResponse {
    queued: bool,
}

impl BuildResponse {
    fn new() -> Self {
        BuildResponse { queued: true }
    }
}

#[post("/v1/deploy/<project_name>/<token>")]
fn deploy(
    tx: State<SyncSender<Job>>,
    projects: State<Projects>,
    project_name: String,
    token: String,
) -> Option<Result<Json<BuildResponse>, Status>> {
    projects
        .get(&project_name)
        .filter(|project| project.token() == token)
        .map(|_| match tx.send(Job::new(project_name)) {
            Ok(_) => Ok(Json(BuildResponse::new())),
            Err(_) => Err(Status::InternalServerError),
        })
}

pub fn start_server() {
    let sender: Sender<Job> = Sender::connect().expect("unable to connect to ipc server");
    let projects = get_projects().expect("unable to get projects");

    let (tx, rx) = sync_channel::<Job>(CHANNEL_BOUND);

    // TODO: implement proper error handling
    thread::spawn(move || loop {
        let job = rx.recv().unwrap();

        sender.send(job).unwrap();
    });

    let config = get_config().expect("unable to read config");
    let listen_config = config.listen();

    let rocket_config = ConfigBuilder::new(Environment::Production)
        .address(listen_config.address())
        .port(listen_config.port())
        .unwrap();

    rocket::custom(rocket_config, true)
        .manage(tx)
        .manage(projects)
        .mount("/", routes![deploy])
        .launch();
}
