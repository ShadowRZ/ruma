use std::time::Duration;

use js_int::UInt;
use ruma_common::OwnedMxcUri;
use serde::{Deserialize, Serialize};

use crate::room::{EncryptedFile, MediaSource};

/// The payload for an audio message.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[serde(tag = "msgtype", rename = "m.audio")]
pub struct AudioMessageEventContent {
    /// The textual representation of this message.
    pub body: String,

    /// The source of the audio clip.
    #[serde(flatten)]
    pub source: MediaSource,

    /// Metadata for the audio clip referred to in `source`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<Box<AudioInfo>>,
}

impl AudioMessageEventContent {
    /// Creates a new `AudioMessageEventContent` with the given body and source.
    pub fn new(body: String, source: MediaSource) -> Self {
        Self { body, source, info: None }
    }

    /// Creates a new non-encrypted `AudioMessageEventContent` with the given bod and url.
    pub fn plain(body: String, url: OwnedMxcUri) -> Self {
        Self::new(body, MediaSource::Plain(url))
    }

    /// Creates a new encrypted `AudioMessageEventContent` with the given body and encrypted
    /// file.
    pub fn encrypted(body: String, file: EncryptedFile) -> Self {
        Self::new(body, MediaSource::Encrypted(Box::new(file)))
    }

    /// Creates a new `AudioMessageEventContent` from `self` with the `info` field set to the given
    /// value.
    ///
    /// Since the field is public, you can also assign to it directly. This method merely acts
    /// as a shorthand for that, because it is very common to set this field.
    pub fn info(self, info: impl Into<Option<Box<AudioInfo>>>) -> Self {
        Self { info: info.into(), ..self }
    }
}

/// Metadata about an audio clip.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct AudioInfo {
    /// The duration of the audio in milliseconds.
    #[serde(
        with = "ruma_common::serde::duration::opt_ms",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration: Option<Duration>,

    /// The mimetype of the audio, e.g. "audio/aac".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,

    /// The size of the audio clip in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<UInt>,
}

impl AudioInfo {
    /// Creates an empty `AudioInfo`.
    pub fn new() -> Self {
        Self::default()
    }
}
