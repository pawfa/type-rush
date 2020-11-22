use crate::engine::environment::environment_type::EnvironmentType;
use std::collections::HashMap;
use crate::engine::environment::environment_record_binding::EnvironmentRecordBinding;
use std::fmt::{Display, Formatter};
use core::fmt;

//it is a dictonary for values in current EnvironmentRecord
pub struct EnvironmentRecord {
    pub env_type: EnvironmentType,
    pub records: HashMap<String, EnvironmentRecordBinding>,
}

impl EnvironmentRecord {
    pub fn new(env_type: EnvironmentType) -> Self {
        let exec = EnvironmentRecord {
            env_type,
            records: HashMap::new(),
        };
        exec
    }
}

impl Display for EnvironmentRecord {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let mut records = String::new();
            for record in &self.records {
                records.push_str( "[");
                records.push_str( "name: ");
                records.push_str( record.0.to_string().as_str());
                records.push_str( ",value:  ");
                records.push_str( record.1.to_string().as_str());
                records.push_str( "]");
            }
            write!(f, "EnvironmentRecord: \n envtype {} \n records: {}",self.env_type, records )
        }
}