use std::sync::{Arc,Mutex};
use std::collections::HashMap;
use std::result::Result;
use lazy_static::lazy_static;
lazy_static!{
    pub static ref REGISTRY: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![]));
}
#[macro_export]
macro_rules! add_to_vec {
    ($left:expr) => {{
        REGISTRY.lock().unwrap().push($left);
    }}
}
pub fn print_registry(){
    for k in &*(REGISTRY.lock().unwrap()){
        println!("{:?}",k);
    }
}