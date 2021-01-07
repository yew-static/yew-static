#[macro_use]
mod macros;

use crate::backend::{ChangeData, InputData};

use cfg_if::cfg_if;
use cfg_match::cfg_match;

cfg_if! {
    if #[cfg(feature = "std_web")] {
        mod listener_stdweb;

        use stdweb::js;
        use stdweb::unstable::{TryFrom, TryInto};
        use stdweb::web::html_element::{InputElement, SelectElement, TextAreaElement};
        use stdweb::web::{Element, EventListenerHandle, FileList, IElement, INode};
        use stdweb::web::event::InputEvent;

        pub use listener_stdweb::*;

        /// Handler to an event listener, only use is to cancel the event.
        #[derive(Debug)]
        pub struct EventListener(Option<EventListenerHandle>);

        impl Drop for EventListener {
            fn drop(&mut self) {
                if let Some(event) = self.0.take() {
                    event.remove()
                }
            }
        }
    } else if #[cfg(feature = "web_sys")] {
        mod listener_web_sys;

        use wasm_bindgen::JsCast;
        use web_sys::{
            Element, FileList, HtmlInputElement as InputElement, HtmlSelectElement as SelectElement,
            HtmlTextAreaElement as TextAreaElement,
            InputEvent
        };

        pub use listener_web_sys::*;

        pub use web_sys::EventListener;

        // TODO: haven't implemented ability to drop an event listener like in stdweb
    } else if #[cfg(feature = "static_render")] {
        use crate::backend::{
            Element, FileList, InputElement, SelectElement,
            TextAreaElement, InputEvent
        };
    }
}

fn oninput_handler(this: &Element, event: InputEvent) -> InputData {
    cfg_if! {
        if #[cfg(feature = "static_render")] {
            unimplemented!();
        } else {
            // Normally only InputElement or TextAreaElement can have an oninput event listener. In
            // practice though any element with `contenteditable=true` may generate such events,
            // therefore here we fall back to just returning the text content of the node.
            // See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event.
            let (v1, v2) = cfg_match! {
                feature = "std_web" => ({
                    (
                        this.clone()
                            .try_into()
                            .map(|input: InputElement| input.raw_value())
                            .ok(),
                        this.clone()
                            .try_into()
                            .map(|input: TextAreaElement| input.value())
                            .ok(),
                    )
                }),
                feature = "web_sys" => ({
                    (
                        this.dyn_ref().map(|input: &InputElement| input.value()),
                        this.dyn_ref().map(|input: &TextAreaElement| input.value()),
                    )
                }),
            };
            let v3 = this.text_content();
            let value = v1.or(v2).or(v3)
                .expect("only an InputElement or TextAreaElement or an element with contenteditable=true can have an oninput event listener");
            InputData { value, event }
        }
    }
}

fn onchange_handler(this: &Element) -> ChangeData {
    cfg_if! {
        if #[cfg(feature = "static_render")] {
            unimplemented!();
        } else {
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
    }
}