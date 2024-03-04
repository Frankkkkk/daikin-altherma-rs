use crate::errors::DAError;
use serde_json::Value;

pub trait FromJsonValue<T>: Sized {
    fn from_json_value(value: &Value) -> Result<T, DAError>;
}

// Implement the trait for i64
impl FromJsonValue<i64> for i64 {
    fn from_json_value(value: &Value) -> Result<Self, DAError> {
        value.as_i64().ok_or(DAError::ValueConversionError)
    }
}

// Implement the trait for f64
impl FromJsonValue<f64> for f64 {
    fn from_json_value(value: &Value) -> Result<Self, DAError> {
        value.as_f64().ok_or(DAError::ValueConversionError)
    }
}

// Implement the trait for String
impl FromJsonValue<String> for String {
    fn from_json_value(value: &Value) -> Result<Self, DAError> {
        let v = value.as_str().ok_or(DAError::ValueConversionError)?;
        Ok(v.to_string())
    }
}

// Implement the trait for bool
impl FromJsonValue<bool> for bool {
    fn from_json_value(value: &Value) -> Result<Self, DAError> {
        value.as_bool().ok_or(DAError::ValueConversionError)
    }
}
