use crate::EnrichedError;
use serde::{Serialize, Serializer};

mod helpers;
mod smoke_tests;
mod syntax_errors;

impl Serialize for EnrichedError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}
