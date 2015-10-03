use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use slackbot::{CommandHandler, Sender};

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
