
pub type Executor = fn(args: Vec<String>) -> bool;

#[derive(Clone)]
pub struct Command {
    name: String,
    usage: Option<String>,
    executor: Executor,
}

impl Command {
    pub fn new(name: String, usage: Option<String>, executor: Executor) -> Self {
        Self {
            name,
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

    pub fn execute(&self, args: Vec<String>) -> bool {
        let executor = self.executor;

        executor(args)
    }
}
