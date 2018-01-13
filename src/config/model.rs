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

impl Project {
    pub fn scripts(&self) -> &[Script] {
        &self.scripts
    }
}

impl Script {
    pub fn command(&self) -> &[String] {
        &self.command
    }

    pub fn allow_failure(&self) -> bool {
        self.allow_failure
    }
}
