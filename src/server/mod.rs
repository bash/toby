use super::worker::{Job, JobTrigger};
use super::config::Config;
use rocket_contrib::Json;
use rocket::{self, State};
use rocket::fairing::AdHoc;
use rocket::request::Form;
use rocket::config::{ConfigBuilder, Environment};
use rocket::http::Status;
use std::sync::mpsc::SyncSender;
use super::status;

#[derive(Serialize, Deserialize)]
struct BuildResponse {
    queued: bool,
}

#[derive(FromForm)]
struct DeployInput {
    token: String,
    secret: String,
}

impl BuildResponse {
    fn new() -> Self {
        BuildResponse { queued: true }
    }
}

#[post("/v1/deploy/<project_name>", data = "<body>")]
fn deploy(
    tx: State<SyncSender<Job>>,
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
            match tx.send(Job::new(
                project_name,
                JobTrigger::Webhook {
                    token: token.into(),
                },
            )) {
                Ok(_) => Ok(Json(BuildResponse::new())),
                Err(_) => Err(Status::InternalServerError),
            }
        })
}

pub fn start_server(config: Config, sender: SyncSender<Job>) {
    let rocket_config = {
        let builder = ConfigBuilder::new(Environment::Production)
            .address(config.main().listen().address().clone())
            .port(config.main().listen().port());

        if let Some(ref tls) = *config.main().tls() {
            builder.tls(tls.certificate(), tls.certificate_key())
        } else {
            builder
        }
    }.unwrap();

    rocket::custom(rocket_config, false)
        .attach(AdHoc::on_launch(|_| status!("Server is starting...")))
        .manage(sender)
        .manage(config)
        .mount("/", routes![deploy])
        .launch();
}
