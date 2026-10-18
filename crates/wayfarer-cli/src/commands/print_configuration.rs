use crate::commands::CommandRunner;
impl CommandRunner {
    pub fn print_configuration(&self) {
        println!("{}", self.config);
    }
}
