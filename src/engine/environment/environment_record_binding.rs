use crate::engine::js_value::JSValue;

//binded code value in current environment
pub struct EnvironmentRecordBinding {
    pub value: Option<JSValue>
}