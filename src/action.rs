use std::str::{self, FromStr};
#[derive(Debug)]
pub enum Action {
    New,
    Delete,
    Quit,
    List,
    Sort,
}

impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" | "new" => Ok(Action::New),
            "d" | "delete" => Ok(Action::Delete),
            "q" | "quit" => Ok(Action::Quit),
            "l" | "list" => Ok(Action::List),
            "s" | "sort" => Ok(Action::Sort),
            _ => Err(s.to_string()),
        }
    }
}
