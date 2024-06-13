use crate::{Error, Message};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum JsonError {
    /// Error during serialization/deserialization.
    #[error("serde_json error")]
    SerdeJson(#[from] serde_json::Error),

    /// The message passed to [`Message::json`] is neither a text nor binary message, and thus can't be deserialized.
    #[error("Can't deserialize message that is neither text nor binary.")]
    NeitherTextNorBinaryMessage,
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        JsonError::from(value).into()
    }
}

impl Message {
    /// Tries to serialize the JSON as a [`Message::Text`].
    ///
    /// # Optional
    ///
    /// This requires the optional `json` feature enabled.
    ///
    /// # Errors
    ///
    /// Serialization can fail if `T`'s implementation of `Serialize` decides to
    /// fail, or if `T` contains a map with non-string keys.
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    pub fn text_from_json<T: Serialize + ?Sized>(json: &T) -> Result<Self, Error> {
        serde_json::to_string(json)
            .map(Message::Text)
            .map_err(Into::into)
    }

    /// Tries to serialize the JSON as a [`Message::Binary`].
    ///
    /// # Optional
    ///
    /// This requires that the optional `json` feature is enabled.
    ///
    /// # Errors
    ///
    /// Serialization can fail if `T`'s implementation of `Serialize` decides to
    /// fail, or if `T` contains a map with non-string keys.
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    pub fn binary_from_json<T: Serialize + ?Sized>(json: &T) -> Result<Self, Error> {
        serde_json::to_vec(json)
            .map(Message::Binary)
            .map_err(Into::into)
    }

    /// Tries to deserialize the message body as JSON.
    ///
    /// # Optional
    ///
    /// This requires that the optional `json` feature is enabled.
    ///
    /// # Errors
    ///
    /// This method fails whenever the response body is not in `JSON` format,
    /// or it cannot be properly deserialized to target type `T`.
    ///
    /// For more details please see [`serde_json::from_str`] and
    /// [`serde_json::from_slice`].
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    pub fn json<T: DeserializeOwned>(&self) -> Result<T, Error> {
        Ok(match self {
            Self::Text(x) => serde_json::from_str(x)?,
            Self::Binary(x) => serde_json::from_slice(x)?,
            _ => return Err(JsonError::NeitherTextNorBinaryMessage.into()),
        })
    }
}
