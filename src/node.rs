use crate::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Node {
    inner: mdast::Node,
    childern: Option<Vec<Node>>,
    folded: bool,
}

impl DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Deref for Node {
    type Target = mdast::Node;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<mdast::Node> for Node {
    fn from(value: mdast::Node) -> Self {
        let childern: Option<Vec<Node>> = value
            .children()
            .map(|v| v.iter().map(Clone::clone).map(Into::into).collect());
        Self {
            inner: value,
            childern,
            folded: true,
        }
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        let mut buf = String::new();
        if let mdast::Node::Text(text) = &self.inner {
            buf.push_str(format!("{}\n", text.value).as_str());
        }
        // buf.push_str(self.inner.to_string().as_str());

        if let Some(childern) = &self.childern {
            for child in childern {
                // buf.push('?');
                buf.push_str(child.to_string().as_str());
            }
        }

        // if let mdast::Node::Heading(head) = &self.inner && self.folded {
        //     format!("{:?}", head)
        // } else {
        //     self.inner.to_string()
        // }
        buf
    }
}

trait OpenBrowser {
    fn open_in_browser(&self) -> Option<std::process::Child>;
}

impl OpenBrowser for mdast::Link {
    fn open_in_browser(&self) -> Option<std::process::Child> {
        std::process::Command::new("xdg-open")
            .arg(&self.url)
            .spawn()
            .ok()
    }
}
