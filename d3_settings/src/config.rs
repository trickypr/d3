//! This is the custom config system for d3_settings. It is not compatible with
//! i3 and will overwrite any existing i3 config if it exists.
//!
//! TODO: Implement some form of config extraction for existing i3 users

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}
