// Copyright 2020-2023 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate rocket;

use ::nym_config::defaults::setup_env;
use clap::{crate_name, crate_version, Parser};
use lazy_static::lazy_static;
use nym_bin_common::build_information::BinaryBuildInformation;
use nym_bin_common::logging::{maybe_print_banner, setup_logging};

mod commands;
mod config;
mod node;

lazy_static! {
    pub static ref PRETTY_BUILD_INFORMATION: String =
        BinaryBuildInformation::new(env!("CARGO_PKG_VERSION")).pretty_print();
}

// Helper for passing LONG_VERSION to clap
fn pretty_build_info_static() -> &'static str {
    &PRETTY_BUILD_INFORMATION
}

#[derive(Parser)]
#[clap(author = "Nymtech", version, about, long_version = pretty_build_info_static())]
struct Cli {
    /// Path pointing to an env file that configures the mixnode.
    #[clap(short, long)]
    pub(crate) config_env_file: Option<std::path::PathBuf>,

    #[clap(subcommand)]
    command: commands::Commands,
}

#[cfg(feature = "cpu-cycles")]
pub fn cpu_cycles() {
    info!("{}", cpu_cycles::cpucycles())
}

#[tokio::main]
async fn main() {
    setup_logging();
    maybe_print_banner(crate_name!(), crate_version!());

    let args = Cli::parse();
    setup_env(args.config_env_file.as_ref());
    commands::execute(args).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
