pub trait RunnableCommand {
    fn run(self, app_name: &'static str);
}
