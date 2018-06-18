#[macro_use]
extern crate serenity;

#[macro_use]
extern crate error_chain;

pub mod error;

use error::*;
use error_chain::ChainedError;
use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::client::Client;
use serenity::framework::standard::{ StandardFramework, help_commands };
use std::env;

fn main() {
    if let Err(why) = launch() {
        let error = Error::from(why);
        eprintln!("{}", error.display_chain().to_string());
    }
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name)
    }
}

fn launch () -> Result<()> {
    let token = env::var("DISCORD_TOKEN")?;

    let mut client = Client::new(&token, Handler)?;

    let framework = StandardFramework::new()
        .configure(|configuration| configuration.on_mention(true))
        .help(help_commands::with_embeds)
        .command("ping", |c| c.cmd(ping))

        .after(|_, _, command_name, result| {
            match result {
                Ok(()) => println!("Processed command '{}'", command_name),
                Err(why) => eprintln!("{:?}", why)
            }
        });

    client.with_framework(framework);

    if let Err(why) = client.start() {
        eprintln!("{}", Error::from(why).display_chain().to_string());
    }

    Ok(())
}

command!(ping(_context, message) {
    Err("wat")?;
    message.reply("Pong")?;
});
