use crate::types::AppInfo;

pub trait RunnableCommand {
    fn run(self, app_info: AppInfo);
}
