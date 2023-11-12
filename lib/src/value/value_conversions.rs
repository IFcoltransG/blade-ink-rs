use super::Value;
use crate::{
    ink_list::InkList,
    object::Object,
    path::Path,
    value_type::{StringValue, ValueType, VariablePointerValue},
};

pub struct ValueTypeMismatch;

impl TryFrom<&Value> for bool {
    type Error = ValueTypeMismatch;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value.value {
            ValueType::Bool(boolean) => Ok(boolean),
            _ => Err(ValueTypeMismatch),
        }
    }
}

impl TryFrom<&Value> for i32 {
    type Error = ValueTypeMismatch;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value.value {
            ValueType::Int(int) => Ok(int),
            _ => Err(ValueTypeMismatch),
        }
    }
}

impl TryFrom<&Value> for f32 {
    type Error = ValueTypeMismatch;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value.value {
            ValueType::Float(float) => Ok(float),
            _ => Err(ValueTypeMismatch),
        }
    }
}

impl<'val> TryFrom<&'val Value> for &'val StringValue {
    type Error = ValueTypeMismatch;

    fn try_from(value: &'val Value) -> Result<Self, Self::Error> {
        match &value.value {
            ValueType::String(string) => Ok(string),
            _ => Err(ValueTypeMismatch),
        }
    }
}

impl<'val> TryFrom<&'val Value> for &'val Path {
    type Error = ValueTypeMismatch;

    fn try_from(value: &'val Value) -> Result<Self, Self::Error> {
        match &value.value {
            ValueType::DivertTarget(target) => Ok(target),
            _ => Err(ValueTypeMismatch),
        }
    }
}

impl<'val> TryFrom<&'val Value> for &'val VariablePointerValue {
    type Error = ValueTypeMismatch;

    fn try_from(value: &'val Value) -> Result<Self, Self::Error> {
        match &value.value {
            ValueType::VariablePointer(pointer) => Ok(pointer),
            _ => Err(ValueTypeMismatch),
        }
    }
}

impl<'val> TryFrom<&'val Value> for &'val InkList {
    type Error = ValueTypeMismatch;

    fn try_from(value: &'val Value) -> Result<Self, Self::Error> {
        match &value.value {
            ValueType::List(list) => Ok(list),
            _ => Err(ValueTypeMismatch),
        }
    }
}

impl<'val> TryFrom<&'val mut Value> for &'val mut InkList {
    type Error = ValueTypeMismatch;

    fn try_from(value: &'val mut Value) -> Result<Self, Self::Error> {
        match &mut value.value {
            ValueType::List(list) => Ok(list),
            _ => Err(ValueTypeMismatch),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self {
            obj: Object::new(),
            value: ValueType::Bool(value),
        }
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self {
            obj: Object::new(),
            value: ValueType::Int(value),
        }
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self {
            obj: Object::new(),
            value: ValueType::Float(value),
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self {
            obj: Object::new(),
            value: ValueType::new_string(value),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self {
            obj: Object::new(),
            value: ValueType::new_string(&value),
        }
    }
}

impl From<Path> for Value {
    fn from(value: Path) -> Self {
        Self {
            obj: Object::new(),
            value: ValueType::DivertTarget(value),
        }
    }
}

impl From<InkList> for Value {
    fn from(value: InkList) -> Self {
        Self {
            obj: Object::new(),
            value: ValueType::List(value),
        }
    }
}

impl From<ValueType> for Value {
    fn from(value_type: ValueType) -> Self {
        Self {
            obj: Object::new(),
            value: value_type,
        }
    }
}
