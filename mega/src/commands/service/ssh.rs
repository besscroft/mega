use clap::{ArgMatches, Args, Command, FromArgMatches};

use common::errors::MegaResult;
use jupiter::context::Context;
use mono::server::ssh_server::start_server;
use mono::server::ssh_server::SshOptions;

pub fn cli() -> Command {
    SshOptions::augment_args_for_update(Command::new("ssh").about("Start Git SSH server"))
}

pub(crate) async fn exec(context: Context, args: &ArgMatches) -> MegaResult {
    let server_matchers = SshOptions::from_arg_matches(args)
        .map_err(|err| err.exit())
        .unwrap();
    tracing::info!("{server_matchers:#?}");
    start_server(context, &server_matchers).await;
    Ok(())
}

#[cfg(test)]
mod tests {}
