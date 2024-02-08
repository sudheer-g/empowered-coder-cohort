mod task;
mod worker;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use crate::task::{Task};
use crate::worker::Worker;

fn main() {
    let mut handles = vec![];
    let (tx, rx) = channel::<Task>();
    let receiver = Arc::new(Mutex::new(rx));
    for i in 1..10 {
        let receiver_clone = Arc::clone(&receiver);
        let handle = thread::spawn(move || {
            let worker = Worker::new(i, receiver_clone);
            worker.accept_tasks();
        });
        handles.push(handle);
    }

    for i  in 1..10 {
        let _task = Task::new(i, "test");
        tx.send(_task).unwrap();
    }

    for handle in handles {
        handle.join().unwrap();
    }




    println!("Hello, world!");
}
