//! This is the main entry point for the Libra.

use mercury::errors::GitError;

mod command;
mod internal;
mod utils;
mod cli;

fn main() {
    #[cfg(debug_assertions)]
    {
        tracing::subscriber::set_global_default(
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .finish(),
        )
        .unwrap();
    }

    let res = cli::parse(None);
    match res {
        Ok(_) => {}
        Err(e) => {
            if let GitError::RepoNotFound = e {
                return;
            } else {
                eprintln!("Error: {:?}", e);
            }
        }
    }
}