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
        if self.registered_users.borrow_mut().remove(&sender.user.name[..]) {
            sender.channel.write(format!("Removed {0} from the daily foosball pool!", sender.user.name)).unwrap();
        } else {
            sender.channel.write(format!("{0} was not in the daily foosball pool.", sender.user.name)).unwrap();
        }
    }
}
