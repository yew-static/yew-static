use anyhow::anyhow;
use web_sys::{
    Document,
    Window,
    Element,
    Node,
    EventListener,
    InputEvent,
    HtmlInputElement as InputElement, 
    HtmlButtonElement as ButtonElement,
    HtmlTextAreaElement as TextAreaElement,
    Text as TextNode
};
use crate::backend::DomBackend;

pub struct Renderer {}

/// A type representing data from `oninput` event.
#[derive(Debug)]
pub struct InputData {
    /// Inserted characters. Contains value from
    /// [InputEvent](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/data).
    pub value: String,
    /// The InputEvent received.
    pub event: InputEvent,
}

impl DomBackend for Renderer {
    type Element = Element;
    type Node = Node;
    type Document = Document;
    type Window = Window;
    type EventListener = EventListener;
    type InputEvent = InputEvent;
    type InputData = InputData;

    type InputElement = InputElement;
    type ButtonElement = ButtonElement;
    type TextAreaElement = TextAreaElement;
    type TextNode = TextNode;

    fn element_as_node(element: &Self::Element) -> Self::Node {
        todo!()
    }

    fn element_last_child(element: &Self::Element) -> Option<Self::Element> {
        todo!()
    }

    fn element_remove_child(element: &Self::Element, child: &Self::Element) -> Result<Self::Node, ()> {
        todo!()
    }

    fn cast_node_ref<INTO>(node_ref: &crate::NodeRef) -> Option<INTO> {
        todo!()
    }

    fn get_document() -> Self::Document {
        Self::get_window().document().unwrap();
        todo!()
    }

    fn get_origin() -> Result<String, anyhow::Error> {
        let location = Self::get_window().location();
        let origin = location.origin().map_err(|e| {
            anyhow!(e
                .as_string()
                .unwrap_or_else(|| String::from("error not recoverable")),)
        })?;
        todo!()
    }

    fn get_host() -> Result<String, anyhow::Error> {
        let location = Self::get_window().location();
        let host = location.host().map_err(|e| {
            anyhow!(e
                .as_string()
                .unwrap_or_else(|| String::from("error not recoverable")),)
        })?;
        todo!()
    }

    fn get_window() -> Self::Window {
        web_sys::window().expect("no window available");
        todo!()
    }
}
