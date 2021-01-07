use anyhow::anyhow;
use wasm_bindgen::JsCast;
use web_sys::{
    Document,
    Window,
    Element,
    Node,
    EventListener,
    InputEvent,
    HtmlInputElement as InputElement, 
    HtmlButtonElement as ButtonElement,
    HtmlSelectElement as SelectElement,
    HtmlTextAreaElement as TextAreaElement,
    Text as TextNode,
    FileList
};
use crate::backend::{DomBackend, base_onchange_handler};

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

// There is no '.../Web/API/ChangeEvent/data' (for onchange) similar to
// https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/data (for oninput).
// ChangeData actually contains the value of the InputElement/TextAreaElement
// after `change` event occured or contains the SelectElement (see more at the
// variant ChangeData::Select)

/// A type representing change of value(s) of an element after committed by user
/// ([onchange event](https://developer.mozilla.org/en-US/docs/Web/Events/change)).
#[derive(Debug)]
pub enum ChangeData {
    /// Value of the element in cases of `<input>`, `<textarea>`
    Value(String),
    /// SelectElement in case of `<select>` element. You can use one of methods of SelectElement
    /// to collect your required data such as: `value`, `selected_index`, `selected_indices` or
    /// `selected_values`. You can also iterate throught `selected_options` yourself.
    Select(SelectElement),
    /// Files
    Files(FileList),
}

impl DomBackend for Renderer {
    type Element = Element;
    type Node = Node;
    type Document = Document;
    type Window = Window;
    type EventListener = EventListener;
    type InputEvent = InputEvent;
    type InputData = InputData;
    type ChangeData = ChangeData;

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

    fn oninput_handler(this: &Self::Element, event: Self::InputEvent) -> Self::InputData {
        let (v1, v2) = (
            this.dyn_ref().map(|input: &InputElement| input.value()),
            this.dyn_ref().map(|input: &TextAreaElement| input.value()),
        );

        let v3 = this.text_content();
        let value = v1.or(v2).or(v3)
            .expect("only an InputElement or TextAreaElement or an element with contenteditable=true can have an oninput event listener");
        InputData { value, event }
    }

    fn onchange_handler(this: &Self::Element) -> Self::ChangeData {
        base_onchange_handler(this)
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
