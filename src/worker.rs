use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use crate::task::Task;

pub struct Worker {
    id: u32,
    queue: Arc<Mutex<Receiver<Task>>>,
}

//TODO implement task timeout and shutdown mechanisms for workers.
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
            let task = self.receive_task();
            &self.process_task(task);
        }
    }

    //TODO This had to be a separate method for the mutex to drop the lock after falling out of scope. Need to understand more on this.
    pub fn receive_task(&self) -> Task {
        let guard = &self.queue.lock().unwrap();
        return guard.recv().unwrap();
    }
    fn process_task(&self, task: Task) -> String {
        thread::sleep(Duration::from_secs(2)); //TODO can be randomised.
        println!("{}", format!("Processing complete. Task Id: {}, Worker Id: {}", task.get_task_id(), &self.id));
        return format!("Processing complete. Task Id: {}, Worker Id: {}", task.get_task_id(), &self.id);
    }
}
