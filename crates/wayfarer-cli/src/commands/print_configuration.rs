use crate::commands::CommandRunner;
impl CommandRunner {
    pub(super) fn print_configuration(&self) {
        println!("{}", self.config);
    }
}
