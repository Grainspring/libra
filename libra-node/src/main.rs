// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use libra_config::config::NodeConfig;
use std::path::PathBuf;
use structopt::StructOpt;
use tracing_subscriber::{prelude::*, registry::Registry};

#[derive(Debug, StructOpt)]
#[structopt(about = "Libra Node")]
struct Args {
    #[structopt(
        short = "f",
        long,
        required_unless = "test",
        help = "Path to NodeConfig"
    )]
    config: Option<PathBuf>,
    #[structopt(long, help = "Enable a single validator testnet")]
    test: bool,
    #[structopt(long, help = "Enabling random ports for testnet")]
    random_ports: bool,
}

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    let args = Args::from_args();

    if args.test {
        println!("Entering test mode, this should never be used in production!");
        libra_node::load_test_environment(args.config, args.random_ports);
    } else {
        let config = NodeConfig::load(args.config.unwrap()).expect("Failed to load node config");
        println!("Using node config {:?}", &config);
        setup_global_subscriber();
        libra_node::start(&config, None);
    };
}

fn setup_global_subscriber() {
    let layer = tracing_atrace::layer().unwrap().with_fields(Option::Some(
        "peer,new_round_event,author,message_round,round,peer_id".to_string(),
    ));
    let subscriber = Registry::default().with(layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
