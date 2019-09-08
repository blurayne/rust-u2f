use std::result::Result;

use hex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_base64::{from_base64, to_base64};
use slog;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct AppId(pub(crate) [u8; 32]);

impl AppId {
    pub fn from_bytes(slice: &[u8]) -> AppId {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(slice);
        AppId(bytes)
    }
}

impl AsRef<[u8]> for AppId {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Serialize for AppId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        to_base64(&self, serializer)
    }
}

impl<'de> Deserialize<'de> for AppId {
    fn deserialize<D>(deserializer: D) -> Result<AppId, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&from_base64(deserializer)?);
        Ok(AppId(bytes))
    }
}

impl slog::Value for AppId {
    fn serialize(
        &self,
        record: &slog::Record,
        key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        slog::Value::serialize(&format!("0x{}", hex::encode_upper(self.0)), record, key, serializer)
    }
}
