use std::{collections::HashMap, ffi::OsString, hash::Hash, time::Duration};

use clap::{Args, Parser};
use futures::{future::BoxFuture, StreamExt};
use libp2p::{noise, ping, swarm::{NetworkBehaviour, SwarmEvent}, tcp, yamux, Swarm};
use tokio::time::sleep;
use tracing_subscriber::EnvFilter;

use crate::{error::Error, global::GLOBAL};

use super::{node::parse_and_run_console_node_command, ConfigArgs};

pub trait Executable {
    fn execute(self) -> Result<(), Error>;
}

pub trait ConsoleCommand {
    fn execute_line(&self, line: String) -> Result<(), Error>;
}
pub struct ConsoleCommands {
    content: HashMap<&'static str, Box<dyn Fn(Vec<String>) -> BoxFuture<'static, Result<(), Error>>>>,
}
impl ConsoleCommands {
    pub fn new() -> Self {
        Self{content: HashMap::new()}
    }
    pub fn insert<F, Fut>(&mut self,name: &'static str, f: F)
    where 
        F: Fn(Vec<String>) -> Fut + 'static,
        Fut: Future<Output = Result<(), Error>> + Send + 'static,
    {
        if let Some(_) = self.content.insert(name, Box::new(move |v| {
            Box::pin(f(v))
        })){
            unreachable!();
        };
    }
    pub async fn parse_and_run(&self, line: String) -> Result<(), Error>{
        let args = shell_words::split(&line)?;
        if let Some(command_name) = args.first().map(|s| {s.clone()}) {
            if let Some(command) = self.content.get(command_name.as_str()) {
                command(args).await
            } else {
                println!("Invalid command: {command_name}");
                self.print_commands();
                Ok(())
            } 
        } else {
            Ok(())
        }
    }
    pub fn print_commands(&self) {
        for key in self.content.keys(){
            println!("{key}");
        }
    }
}

impl Default for ConsoleCommands {
    fn default() -> Self {
        let mut commands = Self::new();
        commands.insert("node", parse_and_run_console_node_command);
        commands
    }
}

#[derive(Args, Debug)]
pub struct ConsoleArgs {
    #[command(flatten)]
    config: ConfigArgs,
}

impl ConsoleArgs {
    pub async fn start_console(self, commands: ConsoleCommands) -> Result<(), Error>
    {
        let _ = crate::global::GLOBAL.get_or_init_node_config(self.config.try_into_node_config().await?).await;
        tokio::spawn( async {
            GLOBAL.launch_swarm().await
        });
        let mut rl = rustyline::DefaultEditor::new()?;
        loop {
            match rl.readline(">> ") {
                Ok(line) => {
                    if let Err(e) = commands.parse_and_run(line).await {                
                        println!("{e}");
                    }
                },
                Err(x) => Err(x)?,
            };
        }
    }
}

