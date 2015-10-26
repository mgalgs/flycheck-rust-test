#[macro_use] extern crate log;
extern crate env_logger;


pub fn sayhello() {
    env_logger::init().unwrap();
    error!("Hello, world!");
}
