use crate::task::Task;
#[derive(Debug, Clone, Default)]
pub struct Tasklist {
    pub tasks: Vec<Task>,
}
impl Tasklist {
    pub fn sort_by_due_date(&mut self) {
        self.tasks.sort_by_key(|task| task.due_date);
    }
    pub fn new(tasks: Vec<Task>) -> Self {
        Tasklist { tasks }
    }
}
