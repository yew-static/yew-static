//! This module contains the implementation of a virtual text node `VText`.

use super::{VDiff, VNode};
use crate::html::{AnyScope, NodeRef};
use crate::backend::{Element, Renderer, TextNode};
use cfg_if::cfg_if;
use log::warn;
use std::borrow::Cow;
use std::cmp::PartialEq;

/// A type for a virtual
/// [`TextNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
/// representation.
#[derive(Clone, Debug)]
pub struct VText {
    /// Contains a text of the node.
    pub text: Cow<'static, str>,
    /// A reference to the `TextNode`.
    pub reference: Option<TextNode>,
}

impl VText {
    /// Creates new virtual text node with a content.
    pub fn new(text: impl Into<Cow<'static, str>>) -> Self {
        VText {
            text: text.into(),
            reference: None,
        }
    }
}

impl VDiff for VText {
    /// Remove VText from parent.
    fn detach(&mut self, parent: &Element) {
        let node = self
            .reference
            .take()
            .expect("tried to remove not rendered VText from DOM");
        if parent.remove_child(&node.into()).is_err() {
            warn!("Node not found to remove VText");
        }
    }

    /// Renders virtual node over existing `TextNode`, but only if value of text has changed.
    fn apply(
        &mut self,
        _parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: Option<VNode>,
    ) -> NodeRef {
        if let Some(mut ancestor) = ancestor {
            if let VNode::VText(mut vtext) = ancestor {
                self.reference = vtext.reference.take();
                let text_node = self
                    .reference
                    .clone()
                    .expect("Rendered VText nodes should have a ref");
                if self.text != vtext.text {
                    text_node.set_node_value(Some(&self.text));
                }

                return NodeRef::new(text_node.into());
            }

            ancestor.detach(parent);
        }

        let text_node = Renderer::get_document().create_text_node(&self.text);
        super::insert_node(&text_node, parent, next_sibling.get());
        let text_node = Renderer::get_document().create_text_node(&self.text);
        super::insert_node((&text_node).into(), parent, next_sibling.get());
        self.reference = Some(text_node.clone());
        NodeRef::new(text_node.into())
    }
}

impl PartialEq for VText {
    fn eq(&self, other: &VText) -> bool {
        self.text == other.text
    }
}
