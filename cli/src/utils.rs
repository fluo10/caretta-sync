use crate::option::ConfigOptionArgs;

pub trait RunnableCommand{
    fn run(self, app_name: &'static str);
}
