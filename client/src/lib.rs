use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use backend::Message;

#[derive(Serialize)]
#[wasm_bindgen]
struct App {
    messages: Vec<Message>,
}

#[derive(Deserialize)]
pub struct Update {
    msg: Msg,
    value: String,
}

#[derive(Deserialize)]
pub enum Msg {
    AddChat,
}

#[wasm_bindgen]
impl App {
    pub fn model(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }

    pub fn update(&self, val: JsValue) -> JsValue {
        let Update { msg, value } = serde_wasm_bindgen::from_value(val).unwrap();

        match msg {
            Msg::AddChat => {
                let message = Message { text: value };
                let messages = [&self.messages[..], &vec![message][..]].concat();

                // TODO: update database

                App { messages }.into()
            }
        }
    }
}

#[wasm_bindgen]
pub fn init() -> JsValue {
    // TODO: read from database
    App {
        messages: Vec::new(),
    }
    .into()
}
