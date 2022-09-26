use crate::FeatureSettings;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Configuration {
    pub name: String,
    pub version: i32,
    pub author: String,
    pub features: FeatureSettings,
}
