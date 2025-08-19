/// Default task data for counter operations
#[derive(Debug)]
pub struct DefaultTaskData {
    #[allow(dead_code)]
    pub var1: String,
    #[allow(dead_code)]
    pub var2: String,
    #[allow(dead_code)]
    pub var3: String,
}

// No trait implementation required in the state-agnostic core.

impl Default for DefaultTaskData {
    fn default() -> Self {
        Self {
            var1: "default_var1".to_string(),
            var2: "default_var2".to_string(),
            var3: "default_var3".to_string(),
        }
    }
}
