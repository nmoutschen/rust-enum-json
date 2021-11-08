use serde::{ser::SerializeStruct, Serialize};

#[derive(Serialize)]
pub struct Data {
    pub id: String,
    pub name: String,
    pub price: f64,
}

/// All possible fields for a response
/// 
/// Some combinations should be impossible, but are not enforced here.
/// It's up to the developer to ensure that the combination is valid.
#[derive(Serialize)]
pub struct AllFields {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
}

/// Optimized response representation
/// 
/// Developers cannot create an invalid response with this enum. However, we
/// have to create a custom serializer to properly transform it into a valid
/// JSON response, but this is an acceptable trade-off to make this library
/// safer to use.
/// 
/// If the success field was a string instead of a bool, we could use the
/// enum tag representation directly.
/// See https://serde.rs/enum-representations.html to learn more.
pub enum ConditionalFields {
    Success(Data),
    Error(String),
}

impl Serialize for ConditionalFields {
    /// Custom serializer for ConditionalFields
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ConditionalFields::Success(data) => {
                let mut s = serializer.serialize_struct("ConditionalFields", 2)?;
                s.serialize_field("success", &true)?;
                s.serialize_field("data", data)?;
                s.end()
            }
            ConditionalFields::Error(error) => {
                let mut s = serializer.serialize_struct("ConditionalFields", 2)?;
                s.serialize_field("success", &false)?;
                s.serialize_field("error", error)?;
                s.end()
            }
        }
    }
}
