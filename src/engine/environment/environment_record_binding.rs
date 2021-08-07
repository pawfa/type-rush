use crate::engine::ts_value::TSValue;
use std::fmt::{Display, Formatter};
use core::fmt;

//binded code value in current environment
pub struct EnvironmentRecordBinding {
    pub value: TSValue
}

impl Display for EnvironmentRecordBinding {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.value {
            ts_val => write!(f, "EnvironmentRecordBinding value: {}", ts_val)
        }

    }
}