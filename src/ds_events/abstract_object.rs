pub mod parser;

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Write};
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AbstractValue {
    Number(i64),
    Symbol(String),
    Object(AbstractObject),
    Map(HashMap<String, AbstractValue>),
    Collection(Vec<AbstractValue>),
}

impl From<i64> for AbstractValue {
    fn from(value: i64) -> Self {
        Self::Number(value)
    }
}

impl From<String> for AbstractValue {
    fn from(value: String) -> Self {
        Self::Symbol(value)
    }
}

impl From<&str> for AbstractValue {
    fn from(value: &str) -> Self {
        Self::Symbol(value.to_string())
    }
}

impl From<AbstractObject> for AbstractValue {
    fn from(value: AbstractObject) -> Self {
        Self::Object(value)
    }
}

impl From<HashMap<String, AbstractValue>> for AbstractValue {
    fn from(value: HashMap<String, AbstractValue>) -> Self {
        Self::Map(value)
    }
}

impl From<Vec<AbstractValue>> for AbstractValue {
    fn from(value: Vec<AbstractValue>) -> Self {
        Self::Collection(value)
    }
}

impl Hash for AbstractValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            AbstractValue::Number(val) => val.hash(state),
            AbstractValue::Symbol(symbol) => symbol.hash(state),
            AbstractValue::Object(obj) => obj.hash(state),
            AbstractValue::Map(values) => {
                // order keys lexicographically
                let mut key_vec = values.keys().collect::<Vec<_>>();
                key_vec.sort();

                // for each key, has the pair of values
                for key in key_vec {
                    let value = values.get(key).unwrap();
                    (key, value).hash(state);
                }
            }
            AbstractValue::Collection(items) => {
                items.iter().for_each(|item| item.hash(state));
            }
        }
    }
}

impl Display for AbstractValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AbstractValue::Number(num) => write!(f, "{}", num),
            AbstractValue::Symbol(symb) => write!(f, "{}", symb),
            AbstractValue::Object(obj) => write!(f, "{}", obj),
            AbstractValue::Map(map) => {
                f.write_char('{')?;
                for (name, value) in map {
                    write!(f, "{}={}, ", name, value)?;
                }
                f.write_char('}')
            }
            AbstractValue::Collection(collect) => {
                f.write_char('[')?;
                for value in collect {
                    write!(f, "{}, ", value)?;
                }
                f.write_char(']')
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct AbstractObject {
    name: String,
    props: HashMap<String, AbstractValue>
}

impl Hash for AbstractObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        // order keys lexicographically
        let mut key_vec = self.props.keys().collect::<Vec<_>>();
        key_vec.sort();

        // for each key, has the pair of values
        for key in key_vec {
            let value = self.props.get(key).unwrap();
            (key, value).hash(state);
        }
    }
}

impl AbstractObject {
    pub fn new<StrT: Into<String>>(name: StrT) -> Self {
        Self {
            name: name.into(),
            props: Default::default()
        }
    }

    fn new_complete<StrT: Into<String>>(name: StrT, props: HashMap<String, AbstractValue>) -> Self {
        Self {
            name: name.into(),
            props
        }
    }

    pub fn with_prop<StrT: Into<String>, ValT: Into<AbstractValue>>(mut self, name: StrT, val: ValT) -> Self {
        self.insert_prop(name, val);
        self
    }

    pub fn insert_prop<StrT: Into<String>, ValT: Into<AbstractValue>>(&mut self, name: StrT, val: ValT) {
        self.props.insert(name.into(), val.into());
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn props(&self) -> &HashMap<String, AbstractValue> {
        &self.props
    }
}

impl Debug for AbstractObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct(self.name());
        for (name, value) in self.props() {
            debug_struct.field(name, value);
        }
        debug_struct.finish()
    }
}

impl Display for AbstractObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.name())?;
        for (name, prop) in self.props() {
            write!(f, "{}={}, ", name, prop)?;
        }
        f.write_char(')')
    }
}

/*
impl Serialize for AbstractObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut obj_map = serializer.serialize_struct(self.name(), self.props().len())?;
        for (name, value) in self.props() {
            obj_map.serialize_field(name, value)?;
        }
        
        obj_map.end()
    }
}
 */
