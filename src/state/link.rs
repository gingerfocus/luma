use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    pub name: String,
    pub link: String,
    pub file: Option<String>,
    pub desc: Option<String>,
    pub artist: Option<String>,
    pub color: Option<String>,
}

// impl Link {
//     pub fn new(name: impl Into<String>, link: impl Into<String>) -> Link {
//         Link {
//             name: name.into(),
//             link: link.into(),
//             file: None,
//             desc: None,
//             artist: None,
//             color: None,
//         }
//     }
//
//     pub fn stub() -> Link {
//         Default::default()
//     }
// }
