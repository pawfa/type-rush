use std::fmt::{Display, Formatter};
use core::fmt;

pub enum EnvironmentType {
    Function,
    Global,
}

impl Display for EnvironmentType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            EnvironmentType::Function => write!(f, "EnvironmentType Function"),
            EnvironmentType::Global => write!(f, "EnvironmentType Global")
        }
    }
}