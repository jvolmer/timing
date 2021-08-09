use chrono::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Start {
    date: DateTime<Local>,
}

impl Start {
    pub fn new(date: DateTime<Local>) -> Self {
        Start { date }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct End {
    date: DateTime<Local>,
}

impl End {
    pub fn new(date: DateTime<Local>) -> Self {
        End { date }
    }
}
