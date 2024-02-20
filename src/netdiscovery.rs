
struct Provider<'a> {
    pub taskList: Vec<&'a dyn NetworkTask>,
}

// struct Provider<'a> {
//     pub taskList: Vec<NetworkTask>,
// }

impl<'a> Provider<'a> {
    fn add_task(&mut self, task: &'a dyn NetworkTask) {
        self.taskList.push(task); // Now modifying taskList is allowed
    }

}




// pub struct Provider {
//     mut taskList: Vec<dyn NetworkTask> 
// }

// pub struct Provider<'a> {
//     pub mut taskList: Vec<&'a dyn NetworkTask> 
// }
 


pub trait NetworkTask {
    fn execute(&self, s: String);
}

// impl<'a> Provider<'a> {
//     pub fn add_task(&self, task: &dyn NetworkTask) {
    
//      //&mut self.taskList.push(task);
//     }
    
//     pub fn execute(&self) { 
        
//     }
// }