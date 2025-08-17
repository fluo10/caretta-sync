use crate::cli::Cli;

mod cli;
mod ipc;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    
    
}
