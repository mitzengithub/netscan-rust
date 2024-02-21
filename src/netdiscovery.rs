
pub trait NetworkTask {
    fn execute(&self, s: String);
}

pub struct Provider<'a> {
    tasklist: Vec<&'a dyn NetworkTask>,
}

// struct Provider<'a> {
//     pub taskList: Vec<NetworkTask>,
// }

impl<'a> Provider<'a> {

    pub fn new(initial_value: Vec<&'a dyn NetworkTask>) -> Self {
        Provider {
            tasklist: initial_value,
        }
    }

    pub fn add_task(&mut self, task: &'a dyn NetworkTask) {
        self.tasklist.push(task); // Now modifying taskList is allowed
    }
    
    pub fn execute(&self)
    {
        for n in self.tasklist.iter() {
            n.execute(String::from("test"));
        }
    }
}

// pub struct Provider {
//     mut taskList: Vec<dyn NetworkTask> 
// }

// pub struct Provider<'a> {
//     pub mut taskList: Vec<&'a dyn NetworkTask> 
// }
 
pub struct PingProvier { 
}

impl NetworkTask for PingProvier {
    fn execute(&self, s: String) {
        println!("executing ping provider from trait: {}", s);
    }
} 

pub struct SqlServerDatabaseConnection { 
}

impl NetworkTask for SqlServerDatabaseConnection {
    fn execute(&self, s: String) {
        println!("executing database connection provider: {}", s);
    }
}

pub struct TraceRouter { 

}

impl NetworkTask for TraceRouter {
    fn execute(&self, s: String) {       
        println!("executing traceroute provider: {}", s);
    }
}



// impl<'a> Provider<'a> {
//     pub fn add_task(&self, task: &dyn NetworkTask) {
    
//      //&mut self.taskList.push(task);
//     }
    
//     pub fn execute(&self) { 
        
//     }
// }