pub struct Task {
    id: u32,
    payload:  *const str,
}

impl Task {
    pub fn new(id: u32, payload: &str) -> Self {
        Task {
            id,
            payload,
        }
    }
    pub fn get_task_id(&self) -> u32 {
        return self.id;
    }
}

unsafe impl Send for Task {}

pub fn create_task(id: u32, payload: &str) -> Task {
    Task {
        id,
        payload,
    }
}