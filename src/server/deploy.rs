use rocket::State;
use rocket::request::Form;
use rocket::http::Status;
use rocket_contrib::Json;
use crate::config::Config;
use crate::worker::{Job, JobTrigger};
use std::sync::mpsc::SyncSender;

#[derive(Serialize, Deserialize)]
pub struct DeployOutput {
    queued: bool,
}

#[derive(FromForm)]
pub struct DeployInput {
    token: String,
    secret: String,
}

impl DeployOutput {
    fn new() -> Self {
        DeployOutput { queued: true }
    }
}

#[post("/v1/deploy/<project_name>", data = "<body>")]
fn deploy(
    tx: State<SyncSender<Job>>,
    config: State<Config>,
    project_name: String,
    body: Form<DeployInput>,
) -> Option<Result<Json<DeployOutput>, Status>> {
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
                Ok(_) => Ok(Json(DeployOutput::new())),
                Err(_) => Err(Status::InternalServerError),
            }
        })
}
