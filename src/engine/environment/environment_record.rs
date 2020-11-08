use crate::engine::environment::environment_type::EnvironmentType;
use std::collections::HashMap;
use crate::engine::environment::environment_record_binding::EnvironmentRecordBinding;

//it is a dictonary for values in current EnvironmentRecord
pub struct EnvironmentRecord {
    pub env_type: EnvironmentType,
    pub records: HashMap<String, EnvironmentRecordBinding>,
}

impl EnvironmentRecord {
    pub fn new(env_type: EnvironmentType) -> Self {
        let mut exec = EnvironmentRecord {
            env_type,
            records: HashMap::new(),
        };
        exec
    }
}