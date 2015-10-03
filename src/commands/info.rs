use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use slackbot::{CommandHandler, Sender};

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
        sender.respond_in_channel(format!("Users in the pool: {:?}", users)).unwrap();
    }
}
