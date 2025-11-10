mod common;
use common::APP_NAME;
use caretta_sync::{
    parser::GuiParser,
    util::RunnableCommand
};
use clap::Parser;

fn main() {
    let args = GuiParser::parse();
    args.run(APP_NAME)
}