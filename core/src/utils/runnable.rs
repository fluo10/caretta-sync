pub trait Runnable {
    fn run(self, app_name: &'static str);
}