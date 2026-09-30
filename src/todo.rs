use std::time::SystemTime;

#[derive(Debug)]
pub enum Status{
    Pending,
    Complete,
}

pub struct Todo{
    pub id: usize,
    pub title: String,
    pub status: Status,
    pub created_at: SystemTime
}

impl Todo {
    pub fn new(id: usize, title: &str) -> Self {
        Self{
            id,
            title: title.to_string(),
            status: Status::Pending,
            created_at: SystemTime::now(),
        }
    }

    pub fn complete(&mut self){
        self.status = Status::Complete;
    }
}
