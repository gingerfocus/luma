pub mod link;

use std::ops::{Deref, DerefMut};

use crate::prelude::*;

// let markdown: Node = markdown::to_mdast(&content, &markdown::ParseOptions::default())
//     .unwrap()
//     .into();

#[derive(Debug)]
pub struct Node {
    inner: mdast::Node,
    // childern: Option<Vec<Node>>,
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
        // let childern: Option<Vec<Node>> = value
        //     .children()
        //     .map(|v| v.iter().map(Clone::clone).map(Into::into).collect());
        Self {
            inner: value,
            // childern,
            folded: true,
        }
    }
}

/// An implementation for displaying the thing with fancy colors
impl PrettyPrint for Node {
    fn to_pstring(&self) -> String {
        format!(
            "
\x1b[0;34mremove\x1b[0m, \x1b[0;34mrm\x1b[0m
\x1b[0;34mrename\x1b[0m, \x1b[0;34mrn\x1b[0m
\x1b[0;34mmove\x1b[0m, \x1b[0;34mmv\x1b[0m
\x1b[0;34mvmove\x1b[0m, \x1b[0;34mvm\x1b[0m
            "
        )
    }
}

/// Just display the thing
impl ToString for Node {
    fn to_string(&self) -> String {
        let buf = String::new();
        to_string_rec(buf, &self.inner)
    }
}

fn children_to_string(
    mut buf: String,
    before: &str,
    children: &Vec<mdast::Node>,
    after: &str,
) -> String {
    buf.push_str(before);
    for child in children {
        buf = to_string_rec(buf, child);
    }
    buf.push_str(after);
    buf
}

macro_rules! call_on_childern {
    ($buf:ident, $node:ident, $before:expr, $after:expr) => {
        $buf = children_to_string($buf, $before, &$node.children, $after)
    };
}

// try doing some sort of depth first search thing where all the nodes are kept
// in memory and the current stack informs how things work.
fn to_string_rec(mut buf: String, node: &mdast::Node) -> String {
    // buf.push_str(self.inner.to_string().as_str());

    // if let Some(pos) = node.inner.position() {
    //     let Position { start, end } = pos;

    // if let mdast::Node::Heading(head) = &self.inner && self.folded {
    //     format!("{:?}", head)
    // } else {
    //     self.inner.to_string()
    // }
    match node {
        // Parents.
        mdast::Node::Root(x) => call_on_childern!(buf, x, "", "\n"),
        mdast::Node::BlockQuote(x) => call_on_childern!(buf, x, "", "\n"),
        // there is still relevant data to be taken out of the footnote thing
        mdast::Node::FootnoteDefinition(x) => call_on_childern!(buf, x, "[1]:", "\n"),
        mdast::Node::List(x) => todo!(),
        mdast::Node::Delete(x) => todo!(),
        mdast::Node::Emphasis(x) => call_on_childern!(buf, x, "*", "*"),
        mdast::Node::Link(x) => call_on_childern!(buf, x, "[", format!("]({})", x.url).as_str()),
        mdast::Node::LinkReference(x) => todo!(),
        mdast::Node::Strong(x) => call_on_childern!(buf, x, "**", "**"),
        mdast::Node::Heading(x) => {
            call_on_childern!(
                buf,
                x,
                (0..x.depth)
                    .map(|_| '#')
                    .chain(vec![' '])
                    .collect::<String>()
                    .as_str(),
                "\n"
            )
        }
        mdast::Node::Table(x) => todo!(),
        mdast::Node::TableRow(x) => todo!(),
        mdast::Node::TableCell(x) => todo!(),
        mdast::Node::ListItem(x) => todo!(),
        mdast::Node::Paragraph(x) => call_on_childern!(buf, x, "", "\n"),
        mdast::Node::MdxJsxFlowElement(x) => todo!(),
        mdast::Node::MdxJsxTextElement(x) => todo!(),

        // Literals.
        mdast::Node::MdxjsEsm(x) => buf.push_str(&x.value),
        mdast::Node::Toml(x) => buf.push_str(&x.value),
        mdast::Node::Yaml(x) => buf.push_str(&x.value),
        mdast::Node::InlineCode(x) => buf.push_str(&x.value),
        mdast::Node::InlineMath(x) => buf.push_str(&x.value),
        mdast::Node::MdxTextExpression(x) => buf.push_str(&x.value),
        mdast::Node::Html(x) => buf.push_str(&x.value),
        mdast::Node::Text(x) => buf.push_str(&x.value),
        mdast::Node::Code(x) => buf.push_str(
            format!(
                "```{}\n{}\n```",
                x.lang.clone().unwrap_or("".into()),
                x.value
            )
            .as_str(),
        ),
        mdast::Node::Math(x) => buf.push_str(&x.value),
        mdast::Node::MdxFlowExpression(x) => buf.push_str(&x.value),

        // Voids.
        mdast::Node::Break(_)
        | mdast::Node::FootnoteReference(_)
        | mdast::Node::Image(_)
        | mdast::Node::ImageReference(_)
        | mdast::Node::ThematicBreak(_)
        | mdast::Node::Definition(_) => {}
    }

    buf
}
