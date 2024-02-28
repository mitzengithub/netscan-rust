
use std::net::{IpAddr};

pub trait NetworkTask {
    fn execute(&self, s: &ProviderModel);
}

pub struct Provider<'a> {
    tasklist: Vec<&'a dyn NetworkTask>,
}

pub struct ProviderModel { 
    pub host: String,
    pub ip: IpAddr
}

impl<'a> Provider<'a> {
    
    pub fn new(initial_value: Vec<&'a dyn NetworkTask>) -> Self {
        Provider {
            tasklist: initial_value,
        }
    }
    
    pub fn add_task(&mut self, task: &'a dyn NetworkTask) {
        self.tasklist.push(task); // Now modifying taskList is allowed
    }
    
    pub fn execute(&self, model: &ProviderModel)
    {
        for n in self.tasklist.iter() {
            n.execute(model);
        }
    }
}


pub struct SqlServerDatabaseConnection { 
}

impl NetworkTask for SqlServerDatabaseConnection {
    fn execute(&self, s: &ProviderModel) {
        println!("executing database connection provider: {}", s.host);
    }
}

