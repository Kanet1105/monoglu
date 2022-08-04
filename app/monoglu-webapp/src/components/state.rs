use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;

pub type SharedState = Rc<UseStateHandle<String>>;

// /// use_refmut equivalent with a State boilerplate.
// pub fn init_shared_state() -> SharedState {
//     Rc::new(use_state(|| State::default()))
// }

// #[derive(Clone, PartialEq)]
// pub struct State {
//     session: RefCell<bool>,
//     user_name: RefCell<String>,
// }

// impl Default for State {
//     fn default() -> Self {
//         Self {
//             session: RefCell::new(false),
//             user_name: RefCell::new(String::from("Sign In")),
//         }
//     }
// }

// impl State {
//     pub fn get_user(&self) -> Result<String, Box<dyn std::error::Error>> {
//         let mut user_name = self.user_name.try_borrow()?;
//         Ok(user_name.to_string())
//     }

//     pub fn set_user(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
//         let mut user_name = self.user_name.try_borrow_mut()?;
//         *user_name = name;
//         Ok(())
//     }
// }