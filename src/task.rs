use chrono::{DateTime, Duration, Local};
use std::fmt;

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub assign_date: DateTime<Local>,
    pub due_date: DateTime<Local>,
}
impl Task {
    pub fn new(name: &str, description: &str, due_date: DateTime<Local>) -> Self {
        let current_time: DateTime<Local> = Local::now();
        Task {
            name: name.to_string(),
            description: description.to_string(),
            assign_date: current_time,
            due_date,
        }
    }
    pub fn time_until_task_due(&self) -> Duration {
        self.due_date - Local::now()
    }
}
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}, Assigned: {}, due {}, time until due: {}",
            self.name,
            self.description,
            self.assign_date.to_rfc2822(),
            self.due_date.to_rfc2822(),
            format_chrono_duration(self.time_until_task_due())
        )
    }
}

pub fn format_chrono_duration(duration: Duration) -> String {
    //Kinda ass if im being honest, but I want a nicer way to format the chrono durations across the codebase
    match duration.num_seconds() {
        0..=59 => format!("{} seconds", duration.num_seconds()),
        60..=3599 => format!("{} minutes", duration.num_minutes()),
        3600..=86399 => format!("{} hours", duration.num_hours()),
        86400..=604799 => format!("{} days", duration.num_days()),
        _ => format!("{} weeks", duration.num_weeks()),
    }
}
