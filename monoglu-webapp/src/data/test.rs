// use std::{
//     sync::{Arc, Mutex},
//     ops::Deref,
// }; 

// pub struct Test{
//     counter1: Arc<Mutex<i32>>,
//     counter2: Arc<Mutext<i32>>,
// }

// impl Test {
//     pub fn new() -> Self {
//         Self{
//             counter1: Arc::new(Mutex::new(0)),
//             counter2: Arc::new(Mutex::new(100)),
//         }
//     }
// }

// impl Clone for Test {
//     fn clone(&self) -> Self {
//         Self{
//             counter1: Arc::clone(&self.0))
//     }
// }