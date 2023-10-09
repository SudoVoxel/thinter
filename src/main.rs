use crate::action::Action;
use crate::task::Task;
use chrono::prelude::*;
use chrono::Local;

use tasklist::Tasklist;

mod action;
mod task;
mod tasklist;

use read_input::prelude::*;
fn main() {
    let mut tasklist: Tasklist = Tasklist::default();

    loop {
        let command: Action = input::<Action>()
            .repeat_msg("(n: new task, d: delete task, q: exit, l: list tasks, s: sort tasks.\n   ")
            .get();

        match command {
            Action::List => list_tasks(tasklist.clone()),
            Action::Delete => delete_task(&mut tasklist),
            Action::Quit => break,
            Action::New => new_task(&mut tasklist),
            Action::Sort => tasklist.sort_by_due_date(),
        }
    }
}

fn list_tasks(tasklist: Tasklist) {
    for task in tasklist.tasks {
        println!("{}", task);
    }
}
fn new_task(tasklist: &mut Tasklist) {
    let name: String = input::<String>().repeat_msg("name:\n").get();
    let description: String = input::<String>().repeat_msg("description:\n").get();

    let due_date: DateTime<Local> = input::<DateTime<Local>>().repeat_msg("due date:\n").get();
    tasklist
        .tasks
        .push(Task::new(name.as_str(), description.as_str(), due_date));
}
fn delete_task(tasklist: &mut Tasklist) {
    let name: String = input::<String>()
        .repeat_msg("name of task to delete:\n")
        .get();
    tasklist.tasks.retain(|x| x.name != name);
}
