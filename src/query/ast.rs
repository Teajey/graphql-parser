//! Query Language Abstract Syntax Tree (AST)
//!
//! The types and fields here resemble official [graphql grammar] whenever it
//! makes sense for rust.
//!
//! [graphql grammar]: http://facebook.github.io/graphql/October2016/#sec-Appendix-Grammar-Summary
//!
use serde::{ser::SerializeMap, Serialize};

pub use crate::common::{Directive, Number, Text, Type, Value};
use crate::position::Pos;

/// Root of query data
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(serde::Serialize))]
pub struct Document<'a, T: Text<'a>> {
    pub definitions: Vec<Definition<'a, T>>,
}

impl<'a> Document<'a, String> {
    pub fn into_static(self) -> Document<'static, String> {
        // To support both reference and owned values in the AST,
        // all string data is represented with the ::common::Str<'a, T: Text<'a>>
        // wrapper type.
        // This type must carry the lifetime of the query string,
        // and is stored in a PhantomData value on the Str type.
        // When using owned String types, the actual lifetime of
        // the Ast nodes is 'static, since no references are kept,
        // but the nodes will still carry the input lifetime.
        // To continue working with Document<String> in a owned fasion
        // the lifetime needs to be transmuted to 'static.
        //
        // This is safe because no references are present.
        // Just the PhantomData lifetime reference is transmuted away.
        unsafe { std::mem::transmute::<_, Document<'static, String>>(self) }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_json", serde(tag = "kind"))]
pub enum Definition<'a, T: Text<'a>> {
    #[cfg_attr(feature = "serde_json", serde(rename = "OperationDefinition"))]
    Operation(OperationDefinition<'a, T>),
    #[cfg_attr(feature = "serde_json", serde(rename = "FragmentDefinition"))]
    Fragment(FragmentDefinition<'a, T>),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_json", serde(rename_all = "camelCase"))]
pub struct FragmentDefinition<'a, T: Text<'a>> {
    #[serde(skip)]
    pub position: Pos,
    #[cfg_attr(
        feature = "serde_json",
        serde(serialize_with = "crate::common::serialize_name")
    )]
    pub name: T::Value,
    pub type_condition: TypeCondition<'a, T>,
    pub directives: Vec<Directive<'a, T>>,
    pub selection_set: SelectionSet<'a, T>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(serde::Serialize))]
#[cfg_attr(
    feature = "serde_json",
    serde(tag = "operation", rename_all = "camelCase")
)]
pub enum OperationDefinition<'a, T: Text<'a>> {
    SelectionSet(SelectionSet<'a, T>),
    Query(Query<'a, T>),
    Mutation(Mutation<'a, T>),
    Subscription(Subscription<'a, T>),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_json", serde(rename_all = "camelCase"))]
pub struct Query<'a, T: Text<'a>> {
    #[serde(skip)]
    pub position: Pos,
    #[cfg_attr(
        feature = "serde_json",
        serde(serialize_with = "crate::common::serialize_optional_name")
    )]
    pub name: Option<T::Value>,
    pub variable_definitions: Vec<VariableDefinition<'a, T>>,
    pub directives: Vec<Directive<'a, T>>,
    pub selection_set: SelectionSet<'a, T>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_json", serde(rename_all = "camelCase"))]
pub struct Mutation<'a, T: Text<'a>> {
    #[serde(skip)]
    pub position: Pos,
    #[cfg_attr(
        feature = "serde_json",
        serde(serialize_with = "crate::common::serialize_optional_name")
    )]
    pub name: Option<T::Value>,
    pub variable_definitions: Vec<VariableDefinition<'a, T>>,
    pub directives: Vec<Directive<'a, T>>,
    pub selection_set: SelectionSet<'a, T>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_json", serde(rename_all = "camelCase"))]
pub struct Subscription<'a, T: Text<'a>> {
    #[serde(skip)]
    pub position: Pos,
    #[cfg_attr(
        feature = "serde_json",
        serde(serialize_with = "crate::common::serialize_optional_name")
    )]
    pub name: Option<T::Value>,
    pub variable_definitions: Vec<VariableDefinition<'a, T>>,
    pub directives: Vec<Directive<'a, T>>,
    pub selection_set: SelectionSet<'a, T>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_json", serde(rename_all = "camelCase"))]
pub struct SelectionSet<'a, T: Text<'a>> {
    #[serde(skip)]
    pub span: (Pos, Pos),
    #[serde(rename = "selections")]
    pub items: Vec<Selection<'a, T>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_json", serde(rename_all = "camelCase"))]
pub struct VariableDefinition<'a, T: Text<'a>> {
    #[serde(skip)]
    pub position: Pos,
    #[cfg_attr(
        feature = "serde_json",
        serde(serialize_with = "crate::common::serialize_name")
    )]
    pub name: T::Value,
    pub var_type: Type<'a, T>,
    pub default_value: Option<Value<'a, T>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_json", serde(tag = "kind"))]
pub enum Selection<'a, T: Text<'a>> {
    Field(Field<'a, T>),
    FragmentSpread(FragmentSpread<'a, T>),
    InlineFragment(InlineFragment<'a, T>),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_json", serde(rename_all = "camelCase"))]
pub struct Field<'a, T: Text<'a>> {
    #[serde(skip)]
    pub position: Pos,
    #[cfg_attr(
        feature = "serde_json",
        serde(serialize_with = "crate::common::serialize_optional_name")
    )]
    pub alias: Option<T::Value>,
    #[cfg_attr(
        feature = "serde_json",
        serde(serialize_with = "crate::common::serialize_name")
    )]
    pub name: T::Value,
    #[cfg_attr(
        feature = "serde_json",
        serde(serialize_with = "crate::common::serialize_arguments")
    )]
    pub arguments: Vec<(T::Value, Value<'a, T>)>,
    pub directives: Vec<Directive<'a, T>>,
    pub selection_set: SelectionSet<'a, T>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_json", serde(rename_all = "camelCase"))]
pub struct FragmentSpread<'a, T: Text<'a>> {
    #[serde(skip)]
    pub position: Pos,
    #[cfg_attr(
        feature = "serde_json",
        serde(serialize_with = "crate::common::serialize_name")
    )]
    pub fragment_name: T::Value,
    pub directives: Vec<Directive<'a, T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeCondition<'a, T: Text<'a>> {
    On(T::Value),
}

impl<'a, T: Text<'a>> Serialize for TypeCondition<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TypeCondition::On(value) => {
                use crate::common::NameKind;
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("kind", "NamedType")?;
                map.serialize_entry(
                    "value",
                    &NameKind {
                        value: value.as_ref(),
                    },
                )?;
                map.end()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_json", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_json", serde(rename_all = "camelCase"))]
pub struct InlineFragment<'a, T: Text<'a>> {
    #[serde(skip)]
    pub position: Pos,
    pub type_condition: Option<TypeCondition<'a, T>>,
    pub directives: Vec<Directive<'a, T>>,
    pub selection_set: SelectionSet<'a, T>,
}
