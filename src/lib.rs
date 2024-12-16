#![doc = include_str!("../README.md")]

#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;

use std::convert::TryFrom;

use derive_getters::Getters;


pub mod ast;
pub mod diagnostic;
mod javadoc;
pub mod parser;
mod rules;
pub mod symbol;
pub mod traverse;
mod validation;

pub use parser::{ParseFileResult, Parser};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(val: JsValue) -> JsValue {
    let aidl_arr: Vec<String> = serde_wasm_bindgen::from_value(val).unwrap();
    let mut parser = Parser::new();

    for (pos, aidl) in aidl_arr.iter().enumerate() {
        parser.add_content(pos, aidl);
        // println!("Element at position {}: {:?}", pos, e);
    }
    let res = parser.validate();
    return JsValue::from_str(serde_json::to_string(&res).unwrap().as_str());
}
