mod data;

pub(crate) use data::ErrorHeapData;

use crate::{
    ecmascript::{
        execution::{agent::ExceptionType, Agent, JsResult},
        types::{
            InternalMethods, IntoObject, IntoValue, Object, OrdinaryObjectInternalSlots,
            PropertyKey, String, Value,
        },
    },
    heap::{indexes::ErrorIndex, GetHeapData},
};

#[derive(Debug, Clone, Copy)]
pub struct Error(pub(crate) ErrorIndex);

impl From<ErrorIndex> for Error {
    fn from(value: ErrorIndex) -> Self {
        Self(value)
    }
}

impl From<ErrorIndex> for Value {
    fn from(value: ErrorIndex) -> Self {
        Self::Error(value)
    }
}

impl IntoValue for Error {
    fn into_value(self) -> Value {
        self.into()
    }
}

impl From<Error> for Value {
    fn from(value: Error) -> Self {
        Value::Error(value.0)
    }
}

impl IntoObject for Error {
    fn into_object(self) -> Object {
        self.into()
    }
}

impl From<Error> for Object {
    fn from(value: Error) -> Self {
        Object::Error(value.0)
    }
}

impl TryFrom<Value> for Error {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, ()> {
        match value {
            Value::Error(idx) => Ok(idx.into()),
            _ => Err(()),
        }
    }
}

impl TryFrom<Object> for Error {
    type Error = ();

    fn try_from(value: Object) -> Result<Self, ()> {
        match value {
            Object::Error(idx) => Ok(idx.into()),
            _ => Err(()),
        }
    }
}

impl OrdinaryObjectInternalSlots for Error {
    fn extensible(self, _agent: &Agent) -> bool {
        false
    }

    fn set_extensible(self, _agent: &mut Agent, _value: bool) {
        todo!()
    }

    fn prototype(self, _agent: &Agent) -> Option<Object> {
        todo!()
    }

    fn set_prototype(self, _agent: &mut Agent, _prototype: Option<Object>) {
        todo!()
    }
}

impl InternalMethods for Error {
    fn get_prototype_of(self, _agent: &mut Agent) -> JsResult<Option<Object>> {
        todo!()
    }

    fn set_prototype_of(self, _agent: &mut Agent, _prototype: Option<Object>) -> JsResult<bool> {
        todo!()
    }

    fn is_extensible(self, _agent: &mut Agent) -> JsResult<bool> {
        todo!()
    }

    fn prevent_extensions(self, _agent: &mut Agent) -> JsResult<bool> {
        todo!()
    }

    fn get_own_property(
        self,
        _agent: &mut Agent,
        _property_key: PropertyKey,
    ) -> JsResult<Option<crate::ecmascript::types::PropertyDescriptor>> {
        todo!()
    }

    fn define_own_property(
        self,
        _agent: &mut Agent,
        _property_key: PropertyKey,
        _property_descriptor: crate::ecmascript::types::PropertyDescriptor,
    ) -> JsResult<bool> {
        todo!()
    }

    fn has_property(self, _agent: &mut Agent, _property_key: PropertyKey) -> JsResult<bool> {
        todo!()
    }

    fn get(self, agent: &mut Agent, property_key: PropertyKey, receiver: Value) -> JsResult<Value> {
        if property_key == PropertyKey::from_str(&mut agent.heap, "toString") {
            agent
                .current_realm()
                .intrinsics()
                .error_prototype()
                .get(agent, property_key, receiver)
        } else if property_key == PropertyKey::from_str(&mut agent.heap, "name") {
            match agent.heap.get(self.0).kind {
                ExceptionType::Error => Ok(String::from_str(agent, "Error").into_value()),
                ExceptionType::EvalError => Ok(String::from_str(agent, "EvalError").into_value()),
                ExceptionType::RangeError => Ok(String::from_str(agent, "RangeError").into_value()),
                ExceptionType::ReferenceError => {
                    Ok(String::from_str(agent, "ReferenceError").into_value())
                }
                ExceptionType::SyntaxError => {
                    Ok(String::from_str(agent, "SyntaxError").into_value())
                }
                ExceptionType::TypeError => Ok(String::from_str(agent, "TypeError").into_value()),
                ExceptionType::UriError => Ok(String::from_str(agent, "UriError").into_value()),
            }
        } else if property_key == PropertyKey::from_str(&mut agent.heap, "message") {
            Ok(agent
                .heap
                .get(self.0)
                .message
                .map_or(Value::Undefined, |message| message.into_value()))
        } else if property_key == PropertyKey::from_str(&mut agent.heap, "cause") {
            Ok(agent.heap.get(self.0).cause.unwrap_or(Value::Undefined))
        } else {
            Ok(Value::Undefined)
        }
    }

    fn set(
        self,
        _agent: &mut Agent,
        _property_key: PropertyKey,
        _value: Value,
        _receiver: Value,
    ) -> JsResult<bool> {
        todo!()
    }

    fn delete(self, _agent: &mut Agent, _property_key: PropertyKey) -> JsResult<bool> {
        todo!()
    }

    fn own_property_keys(self, _agent: &mut Agent) -> JsResult<Vec<PropertyKey>> {
        todo!()
    }
}