use self::token::ValidToken;
use super::config::Config;
use super::status;
use super::worker::{Job, JobTrigger};
use crate::fs::next_job_id;
use crate::worker::WorkerSender;
use rocket::{self, State};
use rocket::config::{ConfigBuilder, Environment};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::Failure;
use rocket_contrib::Json;

mod token;

#[derive(Serialize, Deserialize)]
struct CreateJobResponse {
    id: u64,
}

impl CreateJobResponse {
    fn new(id: u64) -> Self {
        CreateJobResponse { id }
    }
}

#[post("/v1/jobs/<project_name>")]
fn create_job(
    token: ValidToken,
    tx: State<WorkerSender>,
    config: State<Config>,
    project_name: String,
) -> Result<Json<CreateJobResponse>, Failure> {
    let projects = &config.projects;

    match projects
        .get(&project_name)
        .filter(|_| token.can_access(&project_name))
    {
        Some(val) => val,
        None => return Err(Failure(Status::Forbidden)),
    };

    let job_id = match next_job_id(&project_name) {
        Ok(id) => id,
        Err(_) => return Err(Failure(Status::InternalServerError)),
    };

    let job = Job {
        id: job_id,
        project: project_name,
        trigger: JobTrigger::Webhook {
            token: token.token_name().into(),
        },
    };

    match tx.send(job) {
        Ok(_) => Ok(Json(CreateJobResponse::new(job_id))),
        Err(_) => Err(Failure(Status::InternalServerError)),
    }
}

pub fn start_server(config: Config, sender: WorkerSender) {
    #[cfg(not(debug_assertions))]
    let environment = Environment::Production;

    #[cfg(debug_assertions)]
    let environment = Environment::Development;

    let tls_enabled = config.main.tls.is_some();
    let rocket_config = {
        let builder = ConfigBuilder::new(environment)
            .address(config.main.listen.address.clone())
            .port(config.main.listen.port);

        if let Some(ref tls) = config.main.tls {
            builder
                .tls(tls.certificate(), tls.certificate_key())
                .unwrap()
        } else {
            builder.unwrap()
        }
    };

    rocket::custom(rocket_config, false)
        .attach(AdHoc::on_launch(move |rocket| {
            let config = rocket.config();
            let protocol = match tls_enabled {
                true => "https",
                false => "http",
            };

            status!(
                "Server is listening on {}://{}:{}",
                protocol,
                config.address,
                config.port
            );
        }))
        .manage(sender)
        .manage(config)
        .mount("/", routes![create_job])
        .launch();
}
