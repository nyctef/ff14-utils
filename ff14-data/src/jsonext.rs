use serde_json::Value;

/// Helper methods for when we're parsing static data
/// (so we know the structure of the data already follows our assumptions)
pub(crate) trait JsonValueExt {
    fn unwrap_i32(&self, property_name: &str) -> i32;
    fn unwrap_u8(&self, property_name: &str) -> u8;
    fn unwrap_array(&self, property_name: &str) -> &Vec<Value>;
    fn unwrap_object(&self, property_name: &str) -> &Value;
    fn unwrap_str(&self, property_name: &str) -> &str;
    fn unwrap_string(&self, property_name: &str) -> String;
}

impl JsonValueExt for Value {
    fn unwrap_i32(&self, property_name: &str) -> i32 {
        self.get(property_name).unwrap().as_i64().unwrap() as i32
    }

    fn unwrap_u8(&self, property_name: &str) -> u8 {
        self.get(property_name).unwrap().as_u64().unwrap() as u8
    }

    fn unwrap_array(&self, property_name: &str) -> &Vec<Value> {
        self.get(property_name).unwrap().as_array().unwrap()
    }

    fn unwrap_object(&self, property_name: &str) -> &Value {
        self.get(property_name).unwrap() //.as_object().unwrap()
    }

    fn unwrap_str(&self, property_name: &str) -> &str {
        self.get(property_name).unwrap().as_str().unwrap()
    }

    fn unwrap_string(&self, property_name: &str) -> String {
        self.unwrap_str(property_name).to_owned()
    }
}
