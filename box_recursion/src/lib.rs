use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug, Clone)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        Self {
            grade: None,
        }
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        let w = Worker {
            role,
            name,
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(w));
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        let w = self.clone().grade.take().unwrap().name;
        self.grade = self.grade.take().unwrap().next;
        Some(w)
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        let n = self.clone().grade.unwrap().name;
        let r = self.clone().grade.unwrap().role;
        Some((n, r))
    }
}