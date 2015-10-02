use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use rand::{self, Rng};

use slackbot::{CommandHandler,Sender};

pub struct RegisterCommandHandler {
    registered_users: Rc<RefCell<HashSet<String>>>
}

impl RegisterCommandHandler {
    pub fn new(user_store: Rc<RefCell<HashSet<String>>>) -> Self {
        RegisterCommandHandler {
            registered_users: user_store
        }
    }
}

impl CommandHandler for RegisterCommandHandler {
    fn handle(&mut self, sender: &mut Sender, args: &Vec<String>) {
        let username = if args.len() > 0 { args[0].clone() } else { sender.user.name.clone() };
        if self.registered_users.borrow_mut().insert(username.clone()) {
            sender.channel.write(format!("Added {0} to the daily foosball pool!", username)).unwrap();
        } else {
            sender.channel.write(format!("{0} is already in the daily foosball pool.", username)).unwrap();
        }
    }
}

pub struct UnregisterCommandHandler {
    registered_users: Rc<RefCell<HashSet<String>>>
}

impl UnregisterCommandHandler {
    pub fn new(user_store: Rc<RefCell<HashSet<String>>>) -> Self {
        UnregisterCommandHandler {
            registered_users: user_store
        }
    }
}

impl CommandHandler for UnregisterCommandHandler {
    fn handle(&mut self, sender: &mut Sender, _: &Vec<String>) {
        if self.registered_users.borrow_mut().remove(&sender.user.name[..]) {
            sender.channel.write(format!("Removed {0} from the daily foosball pool!", sender.user.name)).unwrap();
        } else {
            sender.channel.write(format!("{0} was not in the daily foosball pool.", sender.user.name)).unwrap();
        }
    }
}

pub struct InfoCommandHandler {
    registered_users: Rc<RefCell<HashSet<String>>>
}

impl InfoCommandHandler {
    pub fn new(user_store: Rc<RefCell<HashSet<String>>>) -> Self {
        InfoCommandHandler {
            registered_users: user_store
        }
    }
}

impl CommandHandler for InfoCommandHandler {
    fn handle(&mut self, sender: &mut Sender, _: &Vec<String>) {
        let users = self.registered_users.borrow();
        sender.channel.write(format!("Users in the pool: {:?}", users)).unwrap();
    }
}

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
            sender.channel.write(format!("Not enough players! Need {0} players to be registered to start a game.", team_size * 2)).unwrap();
        } else {
            let mut users = &mut users.iter().collect::<Vec<_>>()[..];
            let mut rng = rand::thread_rng();
            rng.shuffle(users);
            let team1 = &users[..team_size];
            let team2 = &users[team_size..team_size * 2];
            sender.channel.write(format!("⚽ We have a game! {:?} vs {:?}", team1, team2)).unwrap();
        }
    }
}
