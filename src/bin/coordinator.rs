https://powcoder.com
代写代考加微信 powcoder
Assignment Project Exam Help
Add WeChat powcoder
//! The MapReduce coordinator.
//!
// Do not modify this file.

use clap::Parser;
use map_reduce::{coordinator, log::init_logger};

use anyhow::Result;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    init_logger();
    let args = coordinator::args::Args::parse();
    coordinator::start(args).await
}
