use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use slackbot::{CommandHandler, Sender};

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
        let username = sender.user.name.clone();
        if self.registered_users.borrow_mut().remove(&username[..]) {
            sender.respond_in_channel(format!("Removed {0} from the daily foosball pool!", username)).unwrap();
        } else {
            sender.respond_in_channel(format!("{0} was not in the daily foosball pool.", username)).unwrap();
        }
    }
}
