use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use slackbot::{CommandHandler, Sender};
use rand::{self, Rng};

pub struct GameCommandHandler {
    registered_users: Rc<RefCell<HashSet<String>>>
}

impl GameCommandHandler {
    pub fn new(user_store: Rc<RefCell<HashSet<String>>>) -> Self {
        GameCommandHandler {
            registered_users: user_store
        }
    }
}

impl CommandHandler for GameCommandHandler {
    fn handle(&mut self, sender: &mut Sender, args: &Vec<String>) {
        let team_size = if args.len() > 0 && args[0] == "1v1" { 1 } else { 2 };
        let users = self.registered_users.borrow();
        if users.len() < team_size * 2 {
            sender.respond_in_channel(format!("Not enough players! Need {0} players to be registered to start a game.", team_size * 2)).unwrap();
        } else {
            let mut users = &mut users.iter().collect::<Vec<_>>()[..];
            let mut rng = rand::thread_rng();
            rng.shuffle(users);
            let team1 = &users[..team_size];
            let team2 = &users[team_size..team_size * 2];
            sender.respond_in_channel(format!("⚽ We have a game! {:?} vs {:?}", team1, team2)).unwrap();
        }
    }
}
