// //! This module contains useful utilities to get information about the current document.
// use std::marker::PhantomData;
// use yew::html::ChildrenRenderer;

// use crate::backend::{get_document, get_window, Document, Window};

// /// Returns the current window. This function will panic if there is no available window.
// pub fn window() -> Window {
//     cfg_match! {
//         feature = "std_web" => stdweb::web::window(),
//         feature = "web_sys" => web_sys::window().expect("no window available"),
//         feature = "static_render" => get_window(),
//     }
// }

// /// Returns the current document.
// pub fn document() -> Document {
//     cfg_match! {
//         feature = "std_web" => stdweb::web::document(),
//         feature = "web_sys" => window().document().unwrap(),
//         feature = "static_render" => get_document(),
//     }
// }

// /// Returns the `host` for the current document. Useful for connecting to the server which serves
// /// the app.
// pub fn host() -> Result<String, Error> {
//     let location = document()
//         .location()
//         .ok_or_else(|| anyhow!("can't get location"))?;

//     #[cfg(feature = "std_web")]
//     let host = location.host().map_err(Error::from)?;

//     #[cfg(feature = "web_sys")]
//     let host = location.host().map_err(|e| {
//         anyhow!(e
//             .as_string()
//             .unwrap_or_else(|| String::from("error not recoverable")),)
//     })?;

//     #[cfg(feature = "static_render")]
//     let host = crate::backend::get_host();

//     Ok(host)
// }

// /// Returns the `origin` of the current window.
// pub fn origin() -> Result<String, Error> {
//     let location = window().location();

//     #[cfg(feature = "std_web")]
//     let location = location.ok_or_else(|| anyhow!("can't get location"))?;

//     #[cfg(feature = "std_web")]
//     let origin = location.origin().map_err(Error::from)?;

//     #[cfg(feature = "web_sys")]
//     let origin = location.origin().map_err(|e| {
//         anyhow!(e
//             .as_string()
//             .unwrap_or_else(|| String::from("error not recoverable")),)
//     })?;

//     #[cfg(feature = "static_render")]
//     let origin = crate::backend::get_origin();

//     Ok(origin)
// }

// /// Map IntoIterator<Item=Into<T>> to Iterator<Item=T>
// pub fn into_node_iter<IT, T, R>(it: IT) -> impl Iterator<Item = R>
// where
//     IT: IntoIterator<Item = T>,
//     T: Into<R>,
// {
//     it.into_iter().map(|n| n.into())
// }

// /// A special type necessary for flattening components returned from nested html macros.
// #[derive(Debug)]
// pub struct NodeSeq<IN, OUT>(Vec<OUT>, PhantomData<IN>);

// impl<IN: Into<OUT>, OUT> From<IN> for NodeSeq<IN, OUT> {
//     fn from(val: IN) -> Self {
//         Self(vec![val.into()], PhantomData::default())
//     }
// }

// impl<IN: Into<OUT>, OUT> From<Vec<IN>> for NodeSeq<IN, OUT> {
//     fn from(val: Vec<IN>) -> Self {
//         Self(
//             val.into_iter().map(|x| x.into()).collect(),
//             PhantomData::default(),
//         )
//     }
// }

// impl<IN: Into<OUT>, OUT> From<ChildrenRenderer<IN>> for NodeSeq<IN, OUT> {
//     fn from(val: ChildrenRenderer<IN>) -> Self {
//         Self(
//             val.into_iter().map(|x| x.into()).collect(),
//             PhantomData::default(),
//         )
//     }
// }

// impl<IN, OUT> IntoIterator for NodeSeq<IN, OUT> {
//     type Item = OUT;
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }
