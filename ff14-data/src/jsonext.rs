pub(crate) trait JsonValueExt {
    fn unwrap_i32(&self, property_name: &str) -> i32;
    fn unwrap_u8(&self, property_name: &str) -> u8;
}

impl JsonValueExt for serde_json::value::Value {
    fn unwrap_i32(&self, property_name: &str) -> i32 {
        self.get(property_name).unwrap().as_i64().unwrap() as i32
    }

    fn unwrap_u8(&self, property_name: &str) -> u8 {
        self.get(property_name).unwrap().as_u64().unwrap() as u8
    }
}
