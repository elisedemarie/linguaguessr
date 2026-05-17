use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    French,
    Japanese,
    Arabic,
    Russian,
}
