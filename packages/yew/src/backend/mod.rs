//! When imported, this module represents the selected backend target.
//! This may be selected via feature flag:
//!     - std_web
//!     - web_sys
//!     - static_render

use cfg_if::cfg_if;
use cfg_match::cfg_match;
use wasm_bindgen::JsCast;
use crate::NodeRef;

pub trait DomBackend {
    type ButtonElement;
    type ChangeData;
    type Document;
    type Element;
    type EventListener;
    type InputData;
    type InputElement;
    type InputEvent;
    type Node;
    type TextAreaElement;
    type TextNode;
    type Window;

    /// Returns the current window. This function will panic if there is no available window.
    fn get_window() -> Self::Window;

    /// Returns the current document.
    fn get_document() -> Self::Document;

    /// Returns the `origin` of the current window.
    fn get_origin() -> Result<String, anyhow::Error>;

    /// Returns the `host` for the current document. Useful for connecting to the server which serves the app.
    fn get_host() -> Result<String, anyhow::Error>;

    // Element-related methods
    fn element_as_node(element: &Self::Element) -> Self::Node;
    fn element_last_child(element: &Self::Element) -> Option<Self::Element>;
    fn element_remove_child(
        element: &Self::Element,
        child: &Self::Element,
    ) -> Result<Self::Node, ()>;
    fn cast_node_ref<INTO>(node_ref: &NodeRef) -> Option<INTO>;

    fn oninput_handler(this: &Self::Element, event: Self::InputEvent) -> Self::InputData;
    fn onchange_handler(this: &Self::Element) -> Self::ChangeData;
}

cfg_if! {
    if #[cfg(feature = "web_sys")] {
        use ::web_sys::{FileList, HtmlSelectElement as SelectElement};
        mod web_sys;
        pub use self::web_sys::{ Renderer };
    }
}

// Re-export types from the specific renderer backend

pub type ChangeData = <Renderer as DomBackend>::ChangeData;
pub type EventListener = <Renderer as DomBackend>::EventListener;
pub type Element = <Renderer as DomBackend>::Element;
pub type Node = <Renderer as DomBackend>::Node;
pub type TextNode = <Renderer as DomBackend>::TextNode;
pub type InputData = <Renderer as DomBackend>::InputData;
pub type InputElement = <Renderer as DomBackend>::InputElement;
pub type InputEvent = <Renderer as DomBackend>::InputEvent;
pub type ButtonElement = <Renderer as DomBackend>::ButtonElement;
pub type TextAreaElement = <Renderer as DomBackend>::TextAreaElement;

fn base_onchange_handler(this: &Element) -> ChangeData {
    match this.node_name().as_ref() {
        "INPUT" => {
            let input = cfg_match! {
                feature = "std_web" => InputElement::try_from(this.clone()).unwrap(),
                feature = "web_sys" => this.dyn_ref::<InputElement>().unwrap(),
            };
            let is_file = input
                .get_attribute("type")
                .map(|value| value.eq_ignore_ascii_case("file"))
                .unwrap_or(false);
            if is_file {
                let files: FileList = cfg_match! {
                    feature = "std_web" => js!( return @{input}.files; ).try_into().unwrap(),
                    feature = "web_sys" => input.files().unwrap(),
                };
                ChangeData::Files(files)
            } else {
                cfg_match! {
                    feature = "std_web" => ChangeData::Value(input.raw_value()),
                    feature = "web_sys" => ChangeData::Value(input.value()),
                }
            }
        }
        "TEXTAREA" => {
            let tae = cfg_match! {
                feature = "std_web" => TextAreaElement::try_from(this.clone()).unwrap(),
                feature = "web_sys" => this.dyn_ref::<TextAreaElement>().unwrap(),
            };
            ChangeData::Value(tae.value())
        }
        "SELECT" => {
            let se = cfg_match! {
                feature = "std_web" => SelectElement::try_from(this.clone()).unwrap(),
                feature = "web_sys" => this.dyn_ref::<SelectElement>().unwrap().clone(),
            };
            ChangeData::Select(se)
        }
        _ => {
            panic!("only an InputElement, TextAreaElement or SelectElement can have an onchange event listener");
        }
    }
}
