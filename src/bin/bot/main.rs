#[macro_use]
extern crate serenity;

#[macro_use]
extern crate error_chain;

pub mod error;

use error::*;
use error_chain::ChainedError;
use serenity::prelude::*;
use serenity::model::*;
use serenity::client::Client;
use std::env;

fn main() {
    if let Err(why) = launch() {
        let error = Error::from(why);
        eprintln!("{}", error.display_chain().to_string());
    }
}

struct Handler;

impl EventHandler for Handler { }

fn launch () -> Result<()> {
    let token = env::var("DISCORD_TOKEN")?;

    let mut client = Client::new(&token, Handler)?;

    if let Err(why) = client.start() {
        eprintln!("{}", Error::from(why).display_chain().to_string());
    }

    Ok(())
}
