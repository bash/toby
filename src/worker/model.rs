#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    project: String,
}

impl Job {
    pub fn new<S: Into<String>>(project: S) -> Self {
        let project = project.into();

        Job { project }
    }
}
