use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use rand::{self, Rng};

use slackbot::{CommandHandler,Sender};

pub struct HelpCommandHandler;

impl CommandHandler for HelpCommandHandler {
    fn handle(&mut self, sender: &mut Sender, _: &Vec<String>) {
        // I want this all in one multi-line message as per the Slack docs (https://api.slack.com/docs/formatting),
        // but it doesn't want to work, using a multi-line string, adding in \n, or escaped \n (\\n).
        sender.channel.write("foosbot, here to serve. beep boop.").unwrap();
        sender.channel.write("  !foos || !foos help -- displays the commands").unwrap();
        sender.channel.write("  !foos register -- register to play that day").unwrap();
        sender.channel.write("  !foos unregister -- unregister to play that day, if previously registered").unwrap();
        sender.channel.write("  !foos info -- see who's currently registered").unwrap();
        sender.channel.write("  !foos game -- start a game now").unwrap();
/*
!foos game 3:00pm -- schedule a game at 3pm
!foos stats -- see the current statistics
!foos stats [user1+user2]:[user3+user4]=10:2 -- input a game's stats
*/
    }
}

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
    fn handle(&mut self, sender: &mut Sender, _: &Vec<String>) {
        if self.registered_users.borrow_mut().insert(sender.user.name.clone()) {
            sender.channel.write(format!("Added {0} to the daily foosball pool!", sender.user.name)).unwrap();
        } else {
            sender.channel.write(format!("{0} is already in the daily foosball pool.", sender.user.name)).unwrap();
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
    fn handle(&mut self, sender: &mut Sender, _: &Vec<String>) {
        let users = self.registered_users.borrow();
        if users.len() < 4 {
            sender.channel.write("Not enough players! Need 4 players to be registered to start a game.").unwrap();
        } else {
            let mut users = &mut users.iter().collect::<Vec<_>>()[..];
            let mut rng = rand::thread_rng();
            rng.shuffle(users);
            let team1 = &users[..2];
            let team2 = &users[2..4];
            sender.channel.write(format!("⚽ We have a game! {:?} vs {:?}", team1, team2)).unwrap();
        }
    }
}
