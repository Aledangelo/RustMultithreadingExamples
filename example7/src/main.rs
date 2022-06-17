use fork::{fork, Fork};
use std::process::Command;

fn main() {
    //println!("Hello, world!");


    match fork() {
        Ok(Fork::Parent(child)) => {
            Command::new("echo").arg("hello").output().expect("failed");
            println!("Continuing execution in parent process, new child has pid: {}", child);
        }
        Ok(Fork::Child) => println!("I'm a new child process"),
        Err(_) => println!("Fork failed"),
        
     }
}
