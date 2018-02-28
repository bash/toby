use super::config::Config;
use super::status;
use super::worker::{Job, JobTrigger};
use crate::fs::next_job_id;
use crate::worker::WorkerSender;
use rocket::{self, State};
use rocket::config::{ConfigBuilder, Environment};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::request::Form;
use rocket_contrib::Json;

#[derive(Serialize, Deserialize)]
struct CreateJobResponse {
    id: u64,
}

#[derive(FromForm)]
struct CreateJobForm {
    token: String,
    secret: String,
}

impl CreateJobResponse {
    fn new(id: u64) -> Self {
        CreateJobResponse { id }
    }
}

#[post("/v1/jobs/<project_name>", data = "<body>")]
fn create_job(
    tx: State<WorkerSender>,
    config: State<Config>,
    project_name: String,
    body: Form<CreateJobForm>,
) -> Option<Result<Json<CreateJobResponse>, Status>> {
    let CreateJobForm { token, secret } = body.into_inner();
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
                Ok(_) => Ok(Json(CreateJobResponse::new(job_id))),
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
        .mount("/", routes![create_job])
        .launch();
}
