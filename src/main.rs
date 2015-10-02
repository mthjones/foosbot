extern crate slackbot;
extern crate rand;

use std::env;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use slackbot::{SlackBot, Sender};

mod commands;
use commands::*;

fn main() {
    let token = env::var("FOOSBOT_API_TOKEN").ok().expect("Failed to get FOOSBOT_API_TOKEN environment variable.");

    let user_store = Rc::new(RefCell::new(HashSet::new()));

    let mut foosbot = SlackBot::new("foos", token);

    foosbot.on("help", Box::new(|sender: &mut Sender, _: &Vec<String>| {
        sender.channel.write("foosbot, here to serve. beep boop.").unwrap();
        sender.channel.write("  `!foos || !foos help` -- displays the commands").unwrap();
        sender.channel.write("  `!foos register` -- register to play that day").unwrap();
        sender.channel.write("  `!foos unregister` -- unregister to play that day, if previously registered").unwrap();
        sender.channel.write("  `!foos info` -- see who's currently registered").unwrap();
        sender.channel.write("  `!foos game` -- start a game now").unwrap();
    }));

    foosbot.on("register", Box::new(RegisterCommandHandler::new(user_store.clone())));
    foosbot.on("unregister", Box::new(UnregisterCommandHandler::new(user_store.clone())));
    foosbot.on("info", Box::new(InfoCommandHandler::new(user_store.clone())));
    foosbot.on("game", Box::new(GameCommandHandler::new(user_store.clone())));

    foosbot.run().unwrap();
}
