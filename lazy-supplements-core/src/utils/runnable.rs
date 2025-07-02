pub trait Runnable {
    async fn run(self);
}