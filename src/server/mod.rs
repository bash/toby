use super::worker::{Job, JobTrigger};
use super::config::Config;
use rocket_contrib::Json;
use rocket::{self, State};
use rocket::fairing::AdHoc;
use rocket::request::Form;
use rocket::config::{ConfigBuilder, Environment};
use rocket::http::Status;
use super::status;
use crate::fs::next_job_id;
use crate::worker::WorkerSender;

#[derive(Serialize, Deserialize)]
struct BuildResponse {
    job_id: u64,
}

#[derive(FromForm)]
struct DeployInput {
    token: String,
    secret: String,
}

impl BuildResponse {
    fn new(job_id: u64) -> Self {
        BuildResponse { job_id }
    }
}

#[post("/v1/deploy/<project_name>", data = "<body>")]
fn deploy(
    tx: State<WorkerSender>,
    config: State<Config>,
    project_name: String,
    body: Form<DeployInput>,
) -> Option<Result<Json<BuildResponse>, Status>> {
    let DeployInput { token, secret } = body.into_inner();
    let projects = config.projects();
    let tokens = config.tokens();

    projects
        .get(&project_name)
        .filter(|_| {
            tokens
                .get(&token)
                .filter(|token| token.secret() == secret && token.can_access(&project_name))
                .is_some()
        })
        .map(|_| {
            let job_id = match next_job_id(&project_name) {
                Ok(id) => id,
                Err(_) => return Err(Status::InternalServerError),
            };

            let job = Job {
                id: job_id,
                project: project_name,
                trigger: JobTrigger::Webhook {
                    token: token.into(),
                },
            };

            match tx.send(job) {
                Ok(_) => Ok(Json(BuildResponse::new(job_id))),
                Err(_) => Err(Status::InternalServerError),
            }
        })
}

pub fn start_server(config: Config, sender: WorkerSender) {
    let rocket_config = ConfigBuilder::new(Environment::Production)
        .address(config.main().listen().address().clone())
        .port(config.main().listen().port())
        .unwrap();

    rocket::custom(rocket_config, false)
        .attach(AdHoc::on_launch(|_| status!("Server is starting...")))
        .manage(sender)
        .manage(config)
        .mount("/", routes![deploy])
        .launch();
}
