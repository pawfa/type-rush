use crate::engine::js_value::JSValue;
use std::fmt::{Display, Formatter};
use core::fmt;

//binded code value in current environment
pub struct EnvironmentRecordBinding {
    pub value: Option<JSValue>
}

impl Display for EnvironmentRecordBinding {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.value {
            None => write!(f, "EnvironmentRecordBinding value: None"),
            Some(js_val) => write!(f, "EnvironmentRecordBinding value: {}",js_val)
        }

    }
}