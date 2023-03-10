pub enum TaskStatus {
    DONE,
    PENDING
}

use std::fmt;

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => {"DONE".to_string()},
            &Self::PENDING => {"PENDING".to_string()}
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match &self {
	    &Self::DONE => {write!(f, "DONE")},
	    &Self::PENDING => {write!(f, "PENDING")}
	}
    }
}
