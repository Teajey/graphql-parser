use serde::{Serialize, Serializer};

pub trait Text<'a>: 'a {
    type Value: 'a;
}

pub enum Value<'a, T: Text<'a>> {
    Object(std::collections::BTreeMap<T::Value, Value<'a, T>>),
}

pub fn serialize_arguments<'a, T, S>(
    args: &'a Vec<(T::Value, Value<'a, T>)>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    T: Text<'a>,
    S: Serializer,
{
    unimplemented!()
}

#[derive(Serialize)]
pub struct Directive<'a, T: Text<'a>> {
    #[serde(serialize_with = "serialize_arguments")]
    pub arguments: Vec<(T::Value, Value<'a, T>)>,
}
