use chrono::{DateTime, Utc,Local};

#[derive(Debug)]
pub struct Employee {
    pub name: String,
    pub last_name: String,
    pub email: String,
    created_at: DateTime<Utc>,
    timestamp: DateTime<Local>,
}

pub struct Employer {
    pub name: String,
    pub last_name: String,
    pub email: String,
    company_name: &'static str,
    created_at: DateTime<Utc>,
    timestamp: DateTime<Local>,
}

impl Employee {
    pub fn new(name: String, last_name: String,email: String) -> Employee {
        Employee {
            name,
            last_name,
            email,
            created_at: Utc::now(),
            timestamp: Local::now(),
        }
    }
}

