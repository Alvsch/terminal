
pub type Executor = fn(args: Vec<String>) -> bool;

#[derive(Clone)]
pub struct Command {
    name: String,
    description: Option<String>,
    usage: Option<String>,
    executor: Executor,
}

impl Command {
    pub fn new(name: String, description: Option<String>, usage: Option<String>, executor: Executor) -> Self {
        Self {
            name,
            description,
            usage,
            executor,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_usage(&self) -> Option<String> {
        self.usage.clone()
    }

    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn execute(&self, args: Vec<String>) -> bool {
        let executor = self.executor;

        executor(args)
    }
}
