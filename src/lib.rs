use std::sync::Arc;

use js_sys::Promise;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct MyApi {
    pub(crate) api: Arc<tokio::sync::RwLock<ApiImpl>>,
}

#[wasm_bindgen]
pub struct ApiImpl {
    pub(crate) name: String,
    pub(crate) some_stuff: Arc<tokio::sync::RwLock<u32>>,
}

#[wasm_bindgen]
impl ApiImpl {
    pub fn new(name: String) -> Self {
        Self {
            name,
            some_stuff: Arc::new(RwLock::new(0)),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub async fn do_stuff(&self) -> u32 {
        let mut some_stuff = self.some_stuff.write().await;
        *some_stuff += 1;
        *some_stuff
    }

    pub async fn get_stuff(&self) -> u32 {
        let some_stuff = self.some_stuff.read().await;
        *some_stuff
    }

    pub fn add_game_to_cart(
        &mut self,
        game_item_dbid: Gid,
        // game_group_dbid: Option<Gid>,
        // source: PlayerActionSource,
    ) -> Result<String, String> {
        if game_item_dbid.data == 0 {
            Err("game_group_dbid is none".to_string())
        } else {
            Ok("ok".to_string())
        }
    }
}

#[wasm_bindgen]
impl MyApi {
    pub fn new(name: String) -> Self {
        Self {
            api: Arc::new(RwLock::new(ApiImpl::new(name))),
        }
    }

    pub async fn get_name(&self) -> String {
        let api = self.api.read().await;
        api.get_name()
    }

    pub async fn do_stuff(&self) -> u32 {
        let mut api = self.api.write().await;
        api.do_stuff().await
    }

    pub async fn get_stuff(&self) -> u32 {
        let api = self.api.read().await;
        console_log!("get_stuff");
        api.get_stuff().await
    }

    pub fn add_game_to_cart(
        &mut self,
        game_item_dbid: Gid,
        // game_group_dbid: Option<Gid>,
        // source: PlayerActionSource,
    ) -> Promise {
        console_log!("add_game_to_cart");
        let api = self.api.clone();
        console_log!("add_game_to_cart");
        future_to_promise(async move {
            let mut api = api.write().await;
            let result = api
                .add_game_to_cart(game_item_dbid) //, game_group_dbid, source)
                .to_js_result()?;
            let serializer = serde_wasm_bindgen::Serializer::json_compatible();
            let result = result.serialize(&serializer);
            Ok(result?)
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, tsify::Tsify)]
#[tsify(from_wasm_abi)]
#[serde(tag = "kind", content = "payload")]
#[serde(rename_all = "snake_case")]
pub enum PlayerActionSource {
    Unknown,
    Aig,
    Navigation,
    Search { search_string: String },
}

#[derive(
    Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(
    any(target_arch = "wasm32", target_os = "macos"),
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub struct Gid {
    pub data: u128,
}

use wasm_bindgen::JsValue;

pub type JsResult<T> = Result<T, JSError>;

pub trait ToJSResultTrait<T>: Sized {
    fn to_js_result(self) -> JsResult<T>;

    fn to_js_result_msg(self, msg: &str) -> JsResult<T> {
        match self.to_js_result() {
            Ok(value) => Ok(value),
            Err(err) => Err(JSError {
                message: format!("{}: {}", msg, err.message),
            }),
        }
    }
}

impl<T> ToJSResultTrait<T> for Option<T> {
    fn to_js_result(self) -> JsResult<T> {
        match self {
            Some(value) => Ok(value),
            None => Err(JSError {
                message: "Option is None".to_string(),
            }),
        }
    }
}

impl<T> ToJSResultTrait<T> for Result<T, String> {
    fn to_js_result(self) -> JsResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(JSError { message: err }),
        }
    }
}

impl<T> ToJSResultTrait<T> for Result<T, reqwest::Error> {
    fn to_js_result(self) -> JsResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(JSError {
                message: err.to_string(),
            }),
        }
    }
}

impl From<JSError> for JsValue {
    fn from(error: JSError) -> JsValue {
        serde_wasm_bindgen::to_value(&error).unwrap()
    }
}

use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, tsify::Tsify)]
#[tsify(from_wasm_abi)]
pub struct JSError {
    pub message: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, tsify::Tsify)]
#[tsify(from_wasm_abi)]
pub struct JSAnalyticItem {
    pub order_id: String,
    pub action_type: String,
    pub test_id: Option<i16>,
    pub object: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, tsify::Tsify)]
#[tsify(from_wasm_abi)]
#[serde(tag = "variant", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JSStreamStatus {
    Start,
    Retry(i8),
    GiveUp,
    Error(String),
}
