#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    token: String,
    repository: Option<String>,
    scripts: Vec<Script>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    command: Vec<String>,
    #[serde(default)] allow_failure: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    listen: String,
}
