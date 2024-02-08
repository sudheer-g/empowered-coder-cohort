use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use crate::task::Task;

pub struct Worker {
    id: u32,
    queue: Arc<Mutex<Receiver<Task>>>,
}

impl Worker {
    pub fn new(id: u32, receiver: Arc<Mutex<Receiver<Task>>>) -> Self {
        Worker {
            id,
            queue: receiver,
        }
    }

    pub fn accept_tasks(&self) {
        println!("Worker: {} has started accepting tasks", self.id);
        loop {
            let guard = &self.queue.lock().unwrap();
            let task = guard.recv();
            &self.process_task(task.unwrap());
        }
    }
    fn process_task(&self, task: Task) -> String {
        thread::sleep(Duration::from_secs(2));
        println!("{}", format!("Processing complete. Task Id: {}, Worker Id: {}", task.get_task_id(), &self.id));
        return format!("Processing complete. Task Id: {}, Worker Id: {}", task.get_task_id(), &self.id);
    }
}
