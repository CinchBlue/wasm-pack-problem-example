#![feature(prelude_import)]
#![feature(fmt_helpers_for_derive)]
#![feature(derive_clone_copy)]
#![feature(derive_eq)]
#![feature(structural_match)]
#![feature(coverage_attribute)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;
use tsify::Tsify;
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

pub struct MyApi {
    pub(crate) api: Arc<tokio::sync::RwLock<ApiImpl>>,
}

impl wasm_bindgen::describe::WasmDescribe for MyApi {
    fn describe() {
        use wasm_bindgen::__wbindgen_if_not_std;
        use wasm_bindgen::describe::*;
        inform(RUST_STRUCT);
        inform(5u32);
        inform(77u32);
        inform(121u32);
        inform(65u32);
        inform(112u32);
        inform(105u32);
    }
}

impl wasm_bindgen::convert::IntoWasmAbi for MyApi {
    type Abi = u32;
    fn into_abi(self) -> u32 {
        use wasm_bindgen::__rt::std::boxed::Box;
        use wasm_bindgen::__rt::WasmRefCell;
        Box::into_raw(Box::new(WasmRefCell::new(self))) as u32
    }
}

impl wasm_bindgen::convert::FromWasmAbi for MyApi {
    type Abi = u32;
    unsafe fn from_abi(js: u32) -> Self {
        use wasm_bindgen::__rt::std::boxed::Box;
        use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
        let ptr = js as *mut WasmRefCell<MyApi>;
        assert_not_null(ptr);
        let js = Box::from_raw(ptr);
        (*js).borrow_mut();
        js.into_inner()
    }
}

impl wasm_bindgen::__rt::core::convert::From<MyApi> for wasm_bindgen::JsValue {
    fn from(value: MyApi) -> Self {
        let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))]
        extern "C" {
            fn __wbg_myapi_new(ptr: u32) -> u32;
        }
        unsafe {
            <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                __wbg_myapi_new(ptr),
            )
        }
    }
}
#[cfg(all(
    target_arch = "wasm32",
    not(any(target_os = "emscripten", target_os = "wasi"))
))]

const _: () = {
    #[no_mangle]
    #[doc(hidden)]
    pub unsafe extern "C" fn __wbg_myapi_free(ptr: u32) {
        let _ = <MyApi as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr);
    }
};

impl wasm_bindgen::convert::RefFromWasmAbi for MyApi {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::Ref<'static, MyApi>;
    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<MyApi>;
        wasm_bindgen::__rt::assert_not_null(js);
        (*js).borrow()
    }
}

impl wasm_bindgen::convert::RefMutFromWasmAbi for MyApi {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RefMut<'static, MyApi>;
    unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<MyApi>;
        wasm_bindgen::__rt::assert_not_null(js);
        (*js).borrow_mut()
    }
}

impl wasm_bindgen::convert::LongRefFromWasmAbi for MyApi {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::Ref<'static, MyApi>;
    unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
        <Self as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(js)
    }
}

impl wasm_bindgen::convert::OptionIntoWasmAbi for MyApi {
    #[inline]
    fn none() -> Self::Abi {
        0
    }
}

impl wasm_bindgen::convert::OptionFromWasmAbi for MyApi {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        *abi == 0
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::TryFromJsValue for MyApi {
    type Error = wasm_bindgen::JsValue;
    fn try_from_js_value(
        value: wasm_bindgen::JsValue,
    ) -> wasm_bindgen::__rt::std::result::Result<Self, Self::Error> {
        let idx = wasm_bindgen::convert::IntoWasmAbi::into_abi(&value);
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))]
        extern "C" {
            fn __wbg_myapi_unwrap(ptr: u32) -> u32;
        }
        let ptr = unsafe { __wbg_myapi_unwrap(idx) };
        if ptr == 0 {
            wasm_bindgen::__rt::std::result::Result::Err(value)
        } else {
            wasm_bindgen::__rt::std::mem::forget(value);
            unsafe {
                wasm_bindgen::__rt::std::result::Result::Ok(
                    <Self as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr),
                )
            }
        }
    }
}
impl wasm_bindgen::describe::WasmDescribeVector for MyApi {
    fn describe_vector() {
        use wasm_bindgen::describe::*;
        inform(VECTOR);
        inform(NAMED_EXTERNREF);
        inform(5u32);
        inform(77u32);
        inform(121u32);
        inform(65u32);
        inform(112u32);
        inform(105u32);
    }
}
impl wasm_bindgen::convert::VectorIntoWasmAbi for MyApi {
    type Abi = <wasm_bindgen::__rt::std::boxed::Box<
        [wasm_bindgen::JsValue],
    > as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    fn vector_into_abi(vector: wasm_bindgen::__rt::std::boxed::Box<[MyApi]>) -> Self::Abi {
        wasm_bindgen::convert::js_value_vector_into_abi(vector)
    }
}
impl wasm_bindgen::convert::VectorFromWasmAbi for MyApi {
    type Abi = <wasm_bindgen::__rt::std::boxed::Box<
        [wasm_bindgen::JsValue],
    > as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn vector_from_abi(js: Self::Abi) -> wasm_bindgen::__rt::std::boxed::Box<[MyApi]> {
        wasm_bindgen::convert::js_value_vector_from_abi(js)
    }
}
#[cfg(target_arch = "wasm32")]

const _: () = {
    static _INCLUDED_FILES: &[&str] = &[];
    #[link_section = "__wasm_bindgen_unstable"]
    pub static _GENERATED: [u8; 106usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}4\x00\x00\x00\x00\x00\x00\x01\x05MyApi\x00\x00\x00\x01\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
};
pub struct ApiImpl {
    pub(crate) name: String,
    pub(crate) some_stuff: Arc<tokio::sync::RwLock<u32>>,
}

impl wasm_bindgen::describe::WasmDescribe for ApiImpl {
    fn describe() {
        use wasm_bindgen::__wbindgen_if_not_std;
        use wasm_bindgen::describe::*;
        inform(RUST_STRUCT);
        inform(7u32);
        inform(65u32);
        inform(112u32);
        inform(105u32);
        inform(73u32);
        inform(109u32);
        inform(112u32);
        inform(108u32);
    }
}

impl wasm_bindgen::convert::IntoWasmAbi for ApiImpl {
    type Abi = u32;
    fn into_abi(self) -> u32 {
        use wasm_bindgen::__rt::std::boxed::Box;
        use wasm_bindgen::__rt::WasmRefCell;
        Box::into_raw(Box::new(WasmRefCell::new(self))) as u32
    }
}

impl wasm_bindgen::convert::FromWasmAbi for ApiImpl {
    type Abi = u32;
    unsafe fn from_abi(js: u32) -> Self {
        use wasm_bindgen::__rt::std::boxed::Box;
        use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
        let ptr = js as *mut WasmRefCell<ApiImpl>;
        assert_not_null(ptr);
        let js = Box::from_raw(ptr);
        (*js).borrow_mut();
        js.into_inner()
    }
}

impl wasm_bindgen::__rt::core::convert::From<ApiImpl> for wasm_bindgen::JsValue {
    fn from(value: ApiImpl) -> Self {
        let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))]
        extern "C" {
            fn __wbg_apiimpl_new(ptr: u32) -> u32;
        }
        unsafe {
            <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                __wbg_apiimpl_new(ptr),
            )
        }
    }
}
#[cfg(all(
    target_arch = "wasm32",
    not(any(target_os = "emscripten", target_os = "wasi"))
))]

const _: () = {
    #[no_mangle]
    #[doc(hidden)]
    pub unsafe extern "C" fn __wbg_apiimpl_free(ptr: u32) {
        let _ = <ApiImpl as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr);
    }
};

impl wasm_bindgen::convert::RefFromWasmAbi for ApiImpl {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::Ref<'static, ApiImpl>;
    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<ApiImpl>;
        wasm_bindgen::__rt::assert_not_null(js);
        (*js).borrow()
    }
}

impl wasm_bindgen::convert::RefMutFromWasmAbi for ApiImpl {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RefMut<'static, ApiImpl>;
    unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<ApiImpl>;
        wasm_bindgen::__rt::assert_not_null(js);
        (*js).borrow_mut()
    }
}

impl wasm_bindgen::convert::LongRefFromWasmAbi for ApiImpl {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::Ref<'static, ApiImpl>;
    unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
        <Self as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(js)
    }
}

impl wasm_bindgen::convert::OptionIntoWasmAbi for ApiImpl {
    #[inline]
    fn none() -> Self::Abi {
        0
    }
}

impl wasm_bindgen::convert::OptionFromWasmAbi for ApiImpl {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        *abi == 0
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::TryFromJsValue for ApiImpl {
    type Error = wasm_bindgen::JsValue;
    fn try_from_js_value(
        value: wasm_bindgen::JsValue,
    ) -> wasm_bindgen::__rt::std::result::Result<Self, Self::Error> {
        let idx = wasm_bindgen::convert::IntoWasmAbi::into_abi(&value);
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))]
        extern "C" {
            fn __wbg_apiimpl_unwrap(ptr: u32) -> u32;
        }
        let ptr = unsafe { __wbg_apiimpl_unwrap(idx) };
        if ptr == 0 {
            wasm_bindgen::__rt::std::result::Result::Err(value)
        } else {
            wasm_bindgen::__rt::std::mem::forget(value);
            unsafe {
                wasm_bindgen::__rt::std::result::Result::Ok(
                    <Self as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr),
                )
            }
        }
    }
}
impl wasm_bindgen::describe::WasmDescribeVector for ApiImpl {
    fn describe_vector() {
        use wasm_bindgen::describe::*;
        inform(VECTOR);
        inform(NAMED_EXTERNREF);
        inform(7u32);
        inform(65u32);
        inform(112u32);
        inform(105u32);
        inform(73u32);
        inform(109u32);
        inform(112u32);
        inform(108u32);
    }
}
impl wasm_bindgen::convert::VectorIntoWasmAbi for ApiImpl {
    type Abi = <wasm_bindgen::__rt::std::boxed::Box<
        [wasm_bindgen::JsValue],
    > as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    fn vector_into_abi(vector: wasm_bindgen::__rt::std::boxed::Box<[ApiImpl]>) -> Self::Abi {
        wasm_bindgen::convert::js_value_vector_into_abi(vector)
    }
}
impl wasm_bindgen::convert::VectorFromWasmAbi for ApiImpl {
    type Abi = <wasm_bindgen::__rt::std::boxed::Box<
        [wasm_bindgen::JsValue],
    > as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn vector_from_abi(js: Self::Abi) -> wasm_bindgen::__rt::std::boxed::Box<[ApiImpl]> {
        wasm_bindgen::convert::js_value_vector_from_abi(js)
    }
}
#[cfg(target_arch = "wasm32")]

const _: () = {
    static _INCLUDED_FILES: &[&str] = &[];
    #[link_section = "__wasm_bindgen_unstable"]
    pub static _GENERATED: [u8; 108usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}6\x00\x00\x00\x00\x00\x00\x01\x07ApiImpl\x00\x00\x00\x01\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
};
impl ApiImpl {
    pub fn new(name: String) -> Self {
        const _: () = {
            #[export_name = "apiimpl_new"]
            pub unsafe extern "C" fn __wasm_bindgen_generated_ApiImpl_new(
                arg0_1: <<String as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                arg0_2: <<String as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                arg0_3: <<String as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                arg0_4: <<String as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            ) -> wasm_bindgen::convert::WasmRet<
                <ApiImpl as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = {
                    let arg0 = unsafe {
                        <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                            <<String as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                arg0_1,
                                arg0_2,
                                arg0_3,
                                arg0_4,
                            ),
                        )
                    };
                    let _ret = ApiImpl::new(arg0);
                    _ret
                };
                <ApiImpl as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret).into()
            }
        };
        #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))]

        const _: () = {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn __wbindgen_describe_apiimpl_new() {
                use wasm_bindgen::describe::*;
                wasm_bindgen::__rt::link_mem_intrinsics();
                inform(FUNCTION);
                inform(0);
                inform(1u32);
                <String as WasmDescribe>::describe();
                <ApiImpl as WasmDescribe>::describe();
                <ApiImpl as WasmDescribe>::describe();
            }
        };
        #[cfg(target_arch = "wasm32")]

        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 125usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}G\x00\x00\x00\x01\x01\x07ApiImpl\x00\x00\x01\x04name\x00\x03new\x01\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
        };
        Self {
            name,
            some_stuff: Arc::new(RwLock::new(0)),
        }
    }
    pub fn get_name(&self) -> String {
        const _: () = {
            #[export_name = "apiimpl_get_name"]
            pub unsafe extern "C" fn __wasm_bindgen_generated_ApiImpl_get_name(
                me: u32,
            ) -> wasm_bindgen::convert::WasmRet<<String as wasm_bindgen::convert::ReturnWasmAbi>::Abi>
            {
                let _ret = {
                    let me = unsafe {
                        <ApiImpl as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(me)
                    };
                    let me = &*me;
                    let _ret = me.get_name();
                    _ret
                };
                <String as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret).into()
            }
        };
        #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))]

        const _: () = {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn __wbindgen_describe_apiimpl_get_name() {
                use wasm_bindgen::describe::*;
                wasm_bindgen::__rt::link_mem_intrinsics();
                inform(FUNCTION);
                inform(0);
                inform(0u32);
                <String as WasmDescribe>::describe();
                <String as WasmDescribe>::describe();
            }
        };
        #[cfg(target_arch = "wasm32")]

        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 125usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}G\x00\x00\x00\x01\x01\x07ApiImpl\x00\x00\x00\x00\x08get_name\x01\x01\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
        };
        self.name.clone()
    }
    pub async fn do_stuff(&self) -> u32 {
        const _: () = {
            #[export_name = "apiimpl_do_stuff"]
            pub unsafe extern "C" fn __wasm_bindgen_generated_ApiImpl_do_stuff(
                me: u32,
            ) -> wasm_bindgen::convert::WasmRet<
                <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = wasm_bindgen_futures::future_to_promise(async move {
                    {
                        let me = unsafe {
                            <ApiImpl as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(me)
                        };
                        let me = &*me;
                        let _ret = me.do_stuff();
                        <u32 as wasm_bindgen::__rt::IntoJsResult>::into_js_result(_ret.await)
                    }
                })
                .into();
                <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                    .into()
            }
        };
        #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))]

        const _: () = {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn __wbindgen_describe_apiimpl_do_stuff() {
                use wasm_bindgen::describe::*;
                wasm_bindgen::__rt::link_mem_intrinsics();
                inform(FUNCTION);
                inform(0);
                inform(0u32);
                <wasm_bindgen::JsValue as WasmDescribe>::describe();
                <u32 as WasmDescribe>::describe();
            }
        };
        #[cfg(target_arch = "wasm32")]

        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 125usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}G\x00\x00\x00\x01\x01\x07ApiImpl\x00\x00\x00\x01\x08do_stuff\x01\x01\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
        };
        let mut some_stuff = self.some_stuff.write().await;
        *some_stuff += 1;
        *some_stuff
    }
    pub async fn get_stuff(&self) -> u32 {
        const _: () = {
            #[export_name = "apiimpl_get_stuff"]
            pub unsafe extern "C" fn __wasm_bindgen_generated_ApiImpl_get_stuff(
                me: u32,
            ) -> wasm_bindgen::convert::WasmRet<
                <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = wasm_bindgen_futures::future_to_promise(async move {
                    {
                        let me = unsafe {
                            <ApiImpl as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(me)
                        };
                        let me = &*me;
                        let _ret = me.get_stuff();
                        <u32 as wasm_bindgen::__rt::IntoJsResult>::into_js_result(_ret.await)
                    }
                })
                .into();
                <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                    .into()
            }
        };
        #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))]

        const _: () = {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn __wbindgen_describe_apiimpl_get_stuff() {
                use wasm_bindgen::describe::*;
                wasm_bindgen::__rt::link_mem_intrinsics();
                inform(FUNCTION);
                inform(0);
                inform(0u32);
                <wasm_bindgen::JsValue as WasmDescribe>::describe();
                <u32 as WasmDescribe>::describe();
            }
        };
        #[cfg(target_arch = "wasm32")]

        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 126usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}H\x00\x00\x00\x01\x01\x07ApiImpl\x00\x00\x00\x01\tget_stuff\x01\x01\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
        };
        let some_stuff = self.some_stuff.read().await;
        *some_stuff
    }
    pub fn add_dish_to_cart(&mut self, dish_item_wid: Wid) -> Result<String, String> {
        const _: () = {
            #[export_name = "apiimpl_add_dish_to_cart"]
            pub unsafe extern "C" fn __wasm_bindgen_generated_ApiImpl_add_dish_to_cart(
                me: u32,
                arg1_1: <<Wid as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                arg1_2: <<Wid as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                arg1_3: <<Wid as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                arg1_4: <<Wid as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            ) -> wasm_bindgen::convert::WasmRet<
                <Result<String, String> as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = {
                    let mut me = unsafe {
                        <ApiImpl as wasm_bindgen::convert::RefMutFromWasmAbi>::ref_mut_from_abi(me)
                    };
                    let me = &mut *me;
                    let arg1 = unsafe {
                        <Wid as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                            <<Wid as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                arg1_1,
                                arg1_2,
                                arg1_3,
                                arg1_4,
                            ),
                        )
                    };
                    let _ret = me.add_dish_to_cart(arg1);
                    _ret
                };
                <Result<String, String> as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                    .into()
            }
        };
        #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))]

        const _: () = {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn __wbindgen_describe_apiimpl_add_dish_to_cart() {
                use wasm_bindgen::describe::*;
                wasm_bindgen::__rt::link_mem_intrinsics();
                inform(FUNCTION);
                inform(0);
                inform(1u32);
                <Wid as WasmDescribe>::describe();
                <Result<String, String> as WasmDescribe>::describe();
                <Result<String, String> as WasmDescribe>::describe();
            }
        };
        #[cfg(target_arch = "wasm32")]

        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 147usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}]\x00\x00\x00\x01\x01\x07ApiImpl\x00\x00\x01\rdish_item_wid\x00\x10add_dish_to_cart\x01\x01\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
        };
        if dish_item_wid.data == 0 {
            Err("dish_group_wid is none".to_string())
        } else {
            Ok("ok".to_string())
        }
    }
}
#[cfg(target_arch = "wasm32")]

const _: () = {
    static _INCLUDED_FILES: &[&str] = &[];
    #[link_section = "__wasm_bindgen_unstable"]
    pub static _GENERATED: [u8; 96usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}*\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
};
impl MyApi {
    pub fn new(name: String) -> Self {
        const _: () = {
            #[export_name = "myapi_new"]
            pub unsafe extern "C" fn __wasm_bindgen_generated_MyApi_new(
                arg0_1: <<String as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                arg0_2: <<String as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                arg0_3: <<String as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                arg0_4: <<String as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            ) -> wasm_bindgen::convert::WasmRet<<MyApi as wasm_bindgen::convert::ReturnWasmAbi>::Abi>
            {
                let _ret = {
                    let arg0 = unsafe {
                        <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                            <<String as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                arg0_1,
                                arg0_2,
                                arg0_3,
                                arg0_4,
                            ),
                        )
                    };
                    let _ret = MyApi::new(arg0);
                    _ret
                };
                <MyApi as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret).into()
            }
        };
        #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))]

        const _: () = {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn __wbindgen_describe_myapi_new() {
                use wasm_bindgen::describe::*;
                wasm_bindgen::__rt::link_mem_intrinsics();
                inform(FUNCTION);
                inform(0);
                inform(1u32);
                <String as WasmDescribe>::describe();
                <MyApi as WasmDescribe>::describe();
                <MyApi as WasmDescribe>::describe();
            }
        };
        #[cfg(target_arch = "wasm32")]

        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 123usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}E\x00\x00\x00\x01\x01\x05MyApi\x00\x00\x01\x04name\x00\x03new\x01\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
        };
        Self {
            api: Arc::new(RwLock::new(ApiImpl::new(name))),
        }
    }
    pub async fn get_name(&self) -> String {
        const _: () = {
            #[export_name = "myapi_get_name"]
            pub unsafe extern "C" fn __wasm_bindgen_generated_MyApi_get_name(
                me: u32,
            ) -> wasm_bindgen::convert::WasmRet<
                <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = wasm_bindgen_futures::future_to_promise(async move {
                    {
                        let me = unsafe {
                            <MyApi as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(me)
                        };
                        let me = &*me;
                        let _ret = me.get_name();
                        <String as wasm_bindgen::__rt::IntoJsResult>::into_js_result(_ret.await)
                    }
                })
                .into();
                <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                    .into()
            }
        };
        #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))]

        const _: () = {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn __wbindgen_describe_myapi_get_name() {
                use wasm_bindgen::describe::*;
                wasm_bindgen::__rt::link_mem_intrinsics();
                inform(FUNCTION);
                inform(0);
                inform(0u32);
                <wasm_bindgen::JsValue as WasmDescribe>::describe();
                <String as WasmDescribe>::describe();
            }
        };
        #[cfg(target_arch = "wasm32")]

        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 123usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}E\x00\x00\x00\x01\x01\x05MyApi\x00\x00\x00\x01\x08get_name\x01\x01\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
        };
        let api = self.api.read().await;
        api.get_name()
    }
    pub async fn do_stuff(&self) -> u32 {
        const _: () = {
            #[export_name = "myapi_do_stuff"]
            pub unsafe extern "C" fn __wasm_bindgen_generated_MyApi_do_stuff(
                me: u32,
            ) -> wasm_bindgen::convert::WasmRet<
                <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = wasm_bindgen_futures::future_to_promise(async move {
                    {
                        let me = unsafe {
                            <MyApi as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(me)
                        };
                        let me = &*me;
                        let _ret = me.do_stuff();
                        <u32 as wasm_bindgen::__rt::IntoJsResult>::into_js_result(_ret.await)
                    }
                })
                .into();
                <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                    .into()
            }
        };
        #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))]

        const _: () = {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn __wbindgen_describe_myapi_do_stuff() {
                use wasm_bindgen::describe::*;
                wasm_bindgen::__rt::link_mem_intrinsics();
                inform(FUNCTION);
                inform(0);
                inform(0u32);
                <wasm_bindgen::JsValue as WasmDescribe>::describe();
                <u32 as WasmDescribe>::describe();
            }
        };
        #[cfg(target_arch = "wasm32")]

        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 123usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}E\x00\x00\x00\x01\x01\x05MyApi\x00\x00\x00\x01\x08do_stuff\x01\x01\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
        };
        let mut api = self.api.write().await;
        api.do_stuff().await
    }
    pub async fn get_stuff(&self) -> u32 {
        const _: () = {
            #[export_name = "myapi_get_stuff"]
            pub unsafe extern "C" fn __wasm_bindgen_generated_MyApi_get_stuff(
                me: u32,
            ) -> wasm_bindgen::convert::WasmRet<
                <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = wasm_bindgen_futures::future_to_promise(async move {
                    {
                        let me = unsafe {
                            <MyApi as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(me)
                        };
                        let me = &*me;
                        let _ret = me.get_stuff();
                        <u32 as wasm_bindgen::__rt::IntoJsResult>::into_js_result(_ret.await)
                    }
                })
                .into();
                <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                    .into()
            }
        };
        #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))]

        const _: () = {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn __wbindgen_describe_myapi_get_stuff() {
                use wasm_bindgen::describe::*;
                wasm_bindgen::__rt::link_mem_intrinsics();
                inform(FUNCTION);
                inform(0);
                inform(0u32);
                <wasm_bindgen::JsValue as WasmDescribe>::describe();
                <u32 as WasmDescribe>::describe();
            }
        };
        #[cfg(target_arch = "wasm32")]

        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 124usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}F\x00\x00\x00\x01\x01\x05MyApi\x00\x00\x00\x01\tget_stuff\x01\x01\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
        };
        let api = self.api.read().await;
        web_sys::console::log_1(
            &{
                let res = format!("get_stuff");
                res
            }
            .into(),
        );
        api.get_stuff().await
    }
    pub fn add_dish_to_cart(&mut self, dish_item_wid: Wid) -> Promise {
        const _: () = {
            #[export_name = "myapi_add_dish_to_cart"]
            pub unsafe extern "C" fn __wasm_bindgen_generated_MyApi_add_dish_to_cart(
                me: u32,
                arg1_1: <<Wid as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                arg1_2: <<Wid as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                arg1_3: <<Wid as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                arg1_4: <<Wid as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            ) -> wasm_bindgen::convert::WasmRet<
                <Promise as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = {
                    let mut me = unsafe {
                        <MyApi as wasm_bindgen::convert::RefMutFromWasmAbi>::ref_mut_from_abi(me)
                    };
                    let me = &mut *me;
                    let arg1 = unsafe {
                        <Wid as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                            <<Wid as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                arg1_1,
                                arg1_2,
                                arg1_3,
                                arg1_4,
                            ),
                        )
                    };
                    let _ret = me.add_dish_to_cart(arg1);
                    _ret
                };
                <Promise as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret).into()
            }
        };
        #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))]

        const _: () = {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn __wbindgen_describe_myapi_add_dish_to_cart() {
                use wasm_bindgen::describe::*;
                wasm_bindgen::__rt::link_mem_intrinsics();
                inform(FUNCTION);
                inform(0);
                inform(1u32);
                <Wid as WasmDescribe>::describe();
                <Promise as WasmDescribe>::describe();
                <Promise as WasmDescribe>::describe();
            }
        };
        #[cfg(target_arch = "wasm32")]

        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 145usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}[\x00\x00\x00\x01\x01\x05MyApi\x00\x00\x01\rdish_item_wid\x00\x10add_dish_to_cart\x01\x01\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
        };
        web_sys::console::log_1(
            &{
                let res = format!("add_dish_to_cart");
                res
            }
            .into(),
        );
        let api = self.api.clone();
        web_sys::console::log_1(
            &{
                let res = format!("add_dish_to_cart");
                res
            }
            .into(),
        );
        future_to_promise(async move {
            let mut api = api.write().await;
            let result = api.add_dish_to_cart(dish_item_wid).to_js_result()?;
            let serializer = serde_wasm_bindgen::Serializer::json_compatible();
            let result = result.serialize(&serializer);
            Ok(result?)
        })
    }
}
#[cfg(target_arch = "wasm32")]

const _: () = {
    static _INCLUDED_FILES: &[&str] = &[];
    #[link_section = "__wasm_bindgen_unstable"]
    pub static _GENERATED: [u8; 96usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}*\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
};
pub enum AnalyticsActionSource {
    Unknown,
    Asr,
    Navigation,
    Search { search_string: String },
}

impl ::core::fmt::Debug for AnalyticsActionSource {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            AnalyticsActionSource::Unknown => ::core::fmt::Formatter::write_str(f, "Unknown"),
            AnalyticsActionSource::Asr => ::core::fmt::Formatter::write_str(f, "Asr"),
            AnalyticsActionSource::Navigation => ::core::fmt::Formatter::write_str(f, "Navigation"),
            AnalyticsActionSource::Search {
                search_string: __self_0,
            } => ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Search",
                "search_string",
                &__self_0,
            ),
        }
    }
}

impl ::core::clone::Clone for AnalyticsActionSource {
    #[inline]
    fn clone(&self) -> AnalyticsActionSource {
        match self {
            AnalyticsActionSource::Unknown => AnalyticsActionSource::Unknown,
            AnalyticsActionSource::Asr => AnalyticsActionSource::Asr,
            AnalyticsActionSource::Navigation => AnalyticsActionSource::Navigation,
            AnalyticsActionSource::Search {
                search_string: __self_0,
            } => AnalyticsActionSource::Search {
                search_string: ::core::clone::Clone::clone(__self_0),
            },
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;

    impl _serde::Serialize for AnalyticsActionSource {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                AnalyticsActionSource::Unknown => {
                    let mut __struct = _serde::Serializer::serialize_struct(
                        __serializer,
                        "AnalyticsActionSource",
                        1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "kind",
                        &_serde::__private::ser::AdjacentlyTaggedEnumVariant {
                            enum_name: "AnalyticsActionSource",
                            variant_index: 0u32,
                            variant_name: "unknown",
                        },
                    )?;
                    _serde::ser::SerializeStruct::end(__struct)
                }
                AnalyticsActionSource::Asr => {
                    let mut __struct = _serde::Serializer::serialize_struct(
                        __serializer,
                        "AnalyticsActionSource",
                        1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "kind",
                        &_serde::__private::ser::AdjacentlyTaggedEnumVariant {
                            enum_name: "AnalyticsActionSource",
                            variant_index: 1u32,
                            variant_name: "asr",
                        },
                    )?;
                    _serde::ser::SerializeStruct::end(__struct)
                }
                AnalyticsActionSource::Navigation => {
                    let mut __struct = _serde::Serializer::serialize_struct(
                        __serializer,
                        "AnalyticsActionSource",
                        1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "kind",
                        &_serde::__private::ser::AdjacentlyTaggedEnumVariant {
                            enum_name: "AnalyticsActionSource",
                            variant_index: 2u32,
                            variant_name: "navigation",
                        },
                    )?;
                    _serde::ser::SerializeStruct::end(__struct)
                }
                AnalyticsActionSource::Search { ref search_string } => {
                    #[doc(hidden)]
                    struct __AdjacentlyTagged<'__a> {
                        data: (&'__a String,),
                        phantom: _serde::__private::PhantomData<AnalyticsActionSource>,
                    }
                    impl<'__a> _serde::Serialize for __AdjacentlyTagged<'__a> {
                        fn serialize<__S>(
                            &self,
                            __serializer: __S,
                        ) -> _serde::__private::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            #[allow(unused_variables)]
                            let (search_string,) = self.data;
                            let mut __serde_state = _serde::Serializer::serialize_struct(
                                __serializer,
                                "search",
                                0 + 1,
                            )?;
                            _serde::ser::SerializeStruct::serialize_field(
                                &mut __serde_state,
                                "search_string",
                                search_string,
                            )?;
                            _serde::ser::SerializeStruct::end(__serde_state)
                        }
                    }
                    let mut __struct = _serde::Serializer::serialize_struct(
                        __serializer,
                        "AnalyticsActionSource",
                        2,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "kind",
                        &_serde::__private::ser::AdjacentlyTaggedEnumVariant {
                            enum_name: "AnalyticsActionSource",
                            variant_index: 3u32,
                            variant_name: "search",
                        },
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "payload",
                        &__AdjacentlyTagged {
                            data: (search_string,),
                            phantom: _serde::__private::PhantomData::<AnalyticsActionSource>,
                        },
                    )?;
                    _serde::ser::SerializeStruct::end(__struct)
                }
            }
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;

    impl<'de> _serde::Deserialize<'de> for AnalyticsActionSource {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"variant index 0 <= i < 4",
                        )),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "unknown" => _serde::__private::Ok(__Field::__field0),
                        "asr" => _serde::__private::Ok(__Field::__field1),
                        "navigation" => _serde::__private::Ok(__Field::__field2),
                        "search" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                            __value, VARIANTS,
                        )),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"unknown" => _serde::__private::Ok(__Field::__field0),
                        b"asr" => _serde::__private::Ok(__Field::__field1),
                        b"navigation" => _serde::__private::Ok(__Field::__field2),
                        b"search" => _serde::__private::Ok(__Field::__field3),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            ))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["unknown", "asr", "navigation", "search"];
            #[doc(hidden)]
            struct __Seed<'de> {
                field: __Field,
                marker: _serde::__private::PhantomData<AnalyticsActionSource>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::DeserializeSeed<'de> for __Seed<'de> {
                type Value = AnalyticsActionSource;
                fn deserialize<__D>(
                    self,
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self::Value, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    match self.field {
                        __Field::__field0 => {
                            match _serde::Deserializer::deserialize_any(
                                __deserializer,
                                _serde::__private::de::UntaggedUnitVisitor::new(
                                    "AnalyticsActionSource",
                                    "Unknown",
                                ),
                            ) {
                                _serde::__private::Ok(()) => {
                                    _serde::__private::Ok(AnalyticsActionSource::Unknown)
                                }
                                _serde::__private::Err(__err) => _serde::__private::Err(__err),
                            }
                        }
                        __Field::__field1 => {
                            match _serde::Deserializer::deserialize_any(
                                __deserializer,
                                _serde::__private::de::UntaggedUnitVisitor::new(
                                    "AnalyticsActionSource",
                                    "Asr",
                                ),
                            ) {
                                _serde::__private::Ok(()) => {
                                    _serde::__private::Ok(AnalyticsActionSource::Asr)
                                }
                                _serde::__private::Err(__err) => _serde::__private::Err(__err),
                            }
                        }
                        __Field::__field2 => {
                            match _serde::Deserializer::deserialize_any(
                                __deserializer,
                                _serde::__private::de::UntaggedUnitVisitor::new(
                                    "AnalyticsActionSource",
                                    "Navigation",
                                ),
                            ) {
                                _serde::__private::Ok(()) => {
                                    _serde::__private::Ok(AnalyticsActionSource::Navigation)
                                }
                                _serde::__private::Err(__err) => _serde::__private::Err(__err),
                            }
                        }
                        __Field::__field3 => {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                                __ignore,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result
                                {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "search_string" => _serde::__private::Ok(__Field::__field0),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"search_string" => {
                                            _serde::__private::Ok(__Field::__field0)
                                        }
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: _serde::__private::PhantomData<AnalyticsActionSource>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = AnalyticsActionSource;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result
                                {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "struct variant AnalyticsActionSource::Search",
                                    )
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::__private::Option<String> =
                                        _serde::__private::None;
                                    while let _serde::__private::Some(__key) =
                                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                    {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private::Option::is_some(&__field0) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                            "search_string",
                                                        ),
                                                    );
                                                }
                                                __field0 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<String>(
                                                        &mut __map,
                                                    )?,
                                                );
                                            }
                                            _ => {
                                                let _ = _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                )?;
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::__private::Some(__field0) => __field0,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("search_string")?
                                        }
                                    };
                                    _serde::__private::Ok(AnalyticsActionSource::Search {
                                        search_string: __field0,
                                    })
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &["search_string"];
                            _serde::Deserializer::deserialize_any(
                                __deserializer,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<AnalyticsActionSource>,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<AnalyticsActionSource>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = AnalyticsActionSource;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "adjacently tagged enum AnalyticsActionSource",
                    )
                }
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    match {
                        let mut __rk: _serde::__private::Option<
                            _serde::__private::de::TagOrContentField,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__k) =
                            _serde::de::MapAccess::next_key_seed(
                                &mut __map,
                                _serde::__private::de::TagContentOtherFieldVisitor {
                                    tag: "kind",
                                    content: "payload",
                                },
                            )?
                        {
                            match __k {
                                _serde::__private::de::TagContentOtherField::Other => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                    continue;
                                }
                                _serde::__private::de::TagContentOtherField::Tag => {
                                    __rk = _serde::__private::Some(
                                        _serde::__private::de::TagOrContentField::Tag,
                                    );
                                    break;
                                }
                                _serde::__private::de::TagContentOtherField::Content => {
                                    __rk = _serde::__private::Some(
                                        _serde::__private::de::TagOrContentField::Content,
                                    );
                                    break;
                                }
                            }
                        }
                        __rk
                    } {
                        _serde::__private::Some(_serde::__private::de::TagOrContentField::Tag) => {
                            let __field = _serde::de::MapAccess::next_value_seed(
                                &mut __map,
                                _serde::__private::de::AdjacentlyTaggedEnumVariantSeed::<__Field> {
                                    enum_name: "AnalyticsActionSource",
                                    variants: VARIANTS,
                                    fields_enum: _serde::__private::PhantomData,
                                },
                            )?;
                            match {
                                let mut __rk: _serde::__private::Option<
                                    _serde::__private::de::TagOrContentField,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__k) =
                                    _serde::de::MapAccess::next_key_seed(
                                        &mut __map,
                                        _serde::__private::de::TagContentOtherFieldVisitor {
                                            tag: "kind",
                                            content: "payload",
                                        },
                                    )?
                                {
                                    match __k {
                                        _serde::__private::de::TagContentOtherField::Other => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                            continue;
                                        }
                                        _serde::__private::de::TagContentOtherField::Tag => {
                                            __rk = _serde::__private::Some(
                                                _serde::__private::de::TagOrContentField::Tag,
                                            );
                                            break;
                                        }
                                        _serde::__private::de::TagContentOtherField::Content => {
                                            __rk = _serde::__private::Some(
                                                _serde::__private::de::TagOrContentField::Content,
                                            );
                                            break;
                                        }
                                    }
                                }
                                __rk
                            } {
                                _serde::__private::Some(
                                    _serde::__private::de::TagOrContentField::Tag,
                                ) => _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::duplicate_field("kind"),
                                ),
                                _serde::__private::Some(
                                    _serde::__private::de::TagOrContentField::Content,
                                ) => {
                                    let __ret = _serde::de::MapAccess::next_value_seed(
                                        &mut __map,
                                        __Seed {
                                            field: __field,
                                            marker: _serde::__private::PhantomData,
                                            lifetime: _serde::__private::PhantomData,
                                        },
                                    )?;
                                    match {
                                        let mut __rk: _serde::__private::Option<
                                            _serde::__private::de::TagOrContentField,
                                        > = _serde::__private::None;
                                        while let _serde::__private::Some(__k) = _serde::de::MapAccess::next_key_seed(
                                            &mut __map,
                                            _serde::__private::de::TagContentOtherFieldVisitor {
                                                tag: "kind",
                                                content: "payload",
                                            },
                                        )? {
                                            match __k {
                                                _serde::__private::de::TagContentOtherField::Other => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                    continue;
                                                }
                                                _serde::__private::de::TagContentOtherField::Tag => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Tag,
                                                    );
                                                    break;
                                                }
                                                _serde::__private::de::TagContentOtherField::Content => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Content,
                                                    );
                                                    break;
                                                }
                                            }
                                        }
                                        __rk
                                    } {
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Tag,
                                        ) => _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "kind",
                                            ),
                                        ),
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Content,
                                        ) => _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "payload",
                                            ),
                                        ),
                                        _serde::__private::None => _serde::__private::Ok(__ret),
                                    }
                                }
                                _serde::__private::None => match __field {
                                    __Field::__field0 => {
                                        _serde::__private::Ok(AnalyticsActionSource::Unknown)
                                    }
                                    __Field::__field1 => {
                                        _serde::__private::Ok(AnalyticsActionSource::Asr)
                                    }
                                    __Field::__field2 => {
                                        _serde::__private::Ok(AnalyticsActionSource::Navigation)
                                    }
                                    _ => _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::missing_field("payload"),
                                    ),
                                },
                            }
                        }
                        _serde::__private::Some(
                            _serde::__private::de::TagOrContentField::Content,
                        ) => {
                            let __content = _serde::de::MapAccess::next_value::<
                                _serde::__private::de::Content,
                            >(&mut __map)?;
                            match {
                                let mut __rk: _serde::__private::Option<
                                    _serde::__private::de::TagOrContentField,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__k) =
                                    _serde::de::MapAccess::next_key_seed(
                                        &mut __map,
                                        _serde::__private::de::TagContentOtherFieldVisitor {
                                            tag: "kind",
                                            content: "payload",
                                        },
                                    )?
                                {
                                    match __k {
                                        _serde::__private::de::TagContentOtherField::Other => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                            continue;
                                        }
                                        _serde::__private::de::TagContentOtherField::Tag => {
                                            __rk = _serde::__private::Some(
                                                _serde::__private::de::TagOrContentField::Tag,
                                            );
                                            break;
                                        }
                                        _serde::__private::de::TagContentOtherField::Content => {
                                            __rk = _serde::__private::Some(
                                                _serde::__private::de::TagOrContentField::Content,
                                            );
                                            break;
                                        }
                                    }
                                }
                                __rk
                            } {
                                _serde::__private::Some(
                                    _serde::__private::de::TagOrContentField::Tag,
                                ) => {
                                    let __deserializer = _serde::__private::de::ContentDeserializer::<
                                        __A::Error,
                                    >::new(
                                        __content
                                    );
                                    let __ret = match _serde::de::MapAccess::next_value_seed(
                                        &mut __map,
                                        _serde::__private::de::AdjacentlyTaggedEnumVariantSeed::<
                                            __Field,
                                        > {
                                            enum_name: "AnalyticsActionSource",
                                            variants: VARIANTS,
                                            fields_enum: _serde::__private::PhantomData,
                                        },
                                    )? {
                                        __Field::__field0 => {
                                            match _serde::Deserializer::deserialize_any(
                                                __deserializer,
                                                _serde::__private::de::UntaggedUnitVisitor::new(
                                                    "AnalyticsActionSource",
                                                    "Unknown",
                                                ),
                                            ) {
                                                _serde::__private::Ok(()) => _serde::__private::Ok(
                                                    AnalyticsActionSource::Unknown,
                                                ),
                                                _serde::__private::Err(__err) => {
                                                    _serde::__private::Err(__err)
                                                }
                                            }
                                        }
                                        __Field::__field1 => {
                                            match _serde::Deserializer::deserialize_any(
                                                __deserializer,
                                                _serde::__private::de::UntaggedUnitVisitor::new(
                                                    "AnalyticsActionSource",
                                                    "Asr",
                                                ),
                                            ) {
                                                _serde::__private::Ok(()) => _serde::__private::Ok(
                                                    AnalyticsActionSource::Asr,
                                                ),
                                                _serde::__private::Err(__err) => {
                                                    _serde::__private::Err(__err)
                                                }
                                            }
                                        }
                                        __Field::__field2 => {
                                            match _serde::Deserializer::deserialize_any(
                                                __deserializer,
                                                _serde::__private::de::UntaggedUnitVisitor::new(
                                                    "AnalyticsActionSource",
                                                    "Navigation",
                                                ),
                                            ) {
                                                _serde::__private::Ok(()) => _serde::__private::Ok(
                                                    AnalyticsActionSource::Navigation,
                                                ),
                                                _serde::__private::Err(__err) => {
                                                    _serde::__private::Err(__err)
                                                }
                                            }
                                        }
                                        __Field::__field3 => {
                                            #[allow(non_camel_case_types)]
                                            #[doc(hidden)]
                                            enum __Field {
                                                __field0,
                                                __ignore,
                                            }
                                            #[doc(hidden)]
                                            struct __FieldVisitor;
                                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                                type Value = __Field;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private::Formatter,
                                                ) -> _serde::__private::fmt::Result
                                                {
                                                    _serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        "field identifier",
                                                    )
                                                }
                                                fn visit_u64<__E>(
                                                    self,
                                                    __value: u64,
                                                ) -> _serde::__private::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        0u64 => {
                                                            _serde::__private::Ok(__Field::__field0)
                                                        }
                                                        _ => {
                                                            _serde::__private::Ok(__Field::__ignore)
                                                        }
                                                    }
                                                }
                                                fn visit_str<__E>(
                                                    self,
                                                    __value: &str,
                                                ) -> _serde::__private::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        "search_string" => {
                                                            _serde::__private::Ok(__Field::__field0)
                                                        }
                                                        _ => {
                                                            _serde::__private::Ok(__Field::__ignore)
                                                        }
                                                    }
                                                }
                                                fn visit_bytes<__E>(
                                                    self,
                                                    __value: &[u8],
                                                ) -> _serde::__private::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        b"search_string" => {
                                                            _serde::__private::Ok(__Field::__field0)
                                                        }
                                                        _ => {
                                                            _serde::__private::Ok(__Field::__ignore)
                                                        }
                                                    }
                                                }
                                            }
                                            impl<'de> _serde::Deserialize<'de> for __Field {
                                                #[inline]
                                                fn deserialize<__D>(
                                                    __deserializer: __D,
                                                ) -> _serde::__private::Result<Self, __D::Error>
                                                where
                                                    __D: _serde::Deserializer<'de>,
                                                {
                                                    _serde::Deserializer::deserialize_identifier(
                                                        __deserializer,
                                                        __FieldVisitor,
                                                    )
                                                }
                                            }
                                            #[doc(hidden)]
                                            struct __Visitor<'de> {
                                                marker: _serde::__private::PhantomData<
                                                    AnalyticsActionSource,
                                                >,
                                                lifetime: _serde::__private::PhantomData<&'de ()>,
                                            }
                                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                                type Value = AnalyticsActionSource;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private::Formatter,
                                                ) -> _serde::__private::fmt::Result
                                                {
                                                    _serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        "struct variant AnalyticsActionSource::Search",
                                                    )
                                                }
                                                #[inline]
                                                fn visit_map<__A>(
                                                    self,
                                                    mut __map: __A,
                                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                                where
                                                    __A: _serde::de::MapAccess<'de>,
                                                {
                                                    let mut __field0: _serde::__private::Option<
                                                        String,
                                                    > = _serde::__private::None;
                                                    while let _serde::__private::Some(__key) =
                                                        _serde::de::MapAccess::next_key::<__Field>(
                                                            &mut __map,
                                                        )?
                                                    {
                                                        match __key {
                                                            __Field::__field0 => {
                                                                if _serde::__private::Option::is_some(&__field0) {
                                                                    return _serde::__private::Err(
                                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                                            "search_string",
                                                                        ),
                                                                    );
                                                                }
                                                                __field0 = _serde::__private::Some(
                                                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                                );
                                                            }
                                                            _ => {
                                                                let _ = _serde::de::MapAccess::next_value::<
                                                                    _serde::de::IgnoredAny,
                                                                >(&mut __map)?;
                                                            }
                                                        }
                                                    }
                                                    let __field0 = match __field0 {
                                                        _serde::__private::Some(__field0) => {
                                                            __field0
                                                        }
                                                        _serde::__private::None => {
                                                            _serde::__private::de::missing_field(
                                                                "search_string",
                                                            )?
                                                        }
                                                    };
                                                    _serde::__private::Ok(
                                                        AnalyticsActionSource::Search {
                                                            search_string: __field0,
                                                        },
                                                    )
                                                }
                                            }
                                            #[doc(hidden)]
                                            const FIELDS: &'static [&'static str] =
                                                &["search_string"];
                                            _serde::Deserializer::deserialize_any(
                                                __deserializer,
                                                __Visitor {
                                                    marker: _serde::__private::PhantomData::<
                                                        AnalyticsActionSource,
                                                    >,
                                                    lifetime: _serde::__private::PhantomData,
                                                },
                                            )
                                        }
                                    }?;
                                    match {
                                        let mut __rk: _serde::__private::Option<
                                            _serde::__private::de::TagOrContentField,
                                        > = _serde::__private::None;
                                        while let _serde::__private::Some(__k) = _serde::de::MapAccess::next_key_seed(
                                            &mut __map,
                                            _serde::__private::de::TagContentOtherFieldVisitor {
                                                tag: "kind",
                                                content: "payload",
                                            },
                                        )? {
                                            match __k {
                                                _serde::__private::de::TagContentOtherField::Other => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                    continue;
                                                }
                                                _serde::__private::de::TagContentOtherField::Tag => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Tag,
                                                    );
                                                    break;
                                                }
                                                _serde::__private::de::TagContentOtherField::Content => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Content,
                                                    );
                                                    break;
                                                }
                                            }
                                        }
                                        __rk
                                    } {
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Tag,
                                        ) => _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "kind",
                                            ),
                                        ),
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Content,
                                        ) => _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "payload",
                                            ),
                                        ),
                                        _serde::__private::None => _serde::__private::Ok(__ret),
                                    }
                                }
                                _serde::__private::Some(
                                    _serde::__private::de::TagOrContentField::Content,
                                ) => _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::duplicate_field("payload"),
                                ),
                                _serde::__private::None => _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field("kind"),
                                ),
                            }
                        }
                        _serde::__private::None => _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::missing_field("kind"),
                        ),
                    }
                }
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    match _serde::de::SeqAccess::next_element(&mut __seq)? {
                        _serde::__private::Some(__field) => {
                            match _serde::de::SeqAccess::next_element_seed(
                                &mut __seq,
                                __Seed {
                                    field: __field,
                                    marker: _serde::__private::PhantomData,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )? {
                                _serde::__private::Some(__ret) => _serde::__private::Ok(__ret),
                                _serde::__private::None => _serde::__private::Err(
                                    _serde::de::Error::invalid_length(1, &self),
                                ),
                            }
                        }
                        _serde::__private::None => {
                            _serde::__private::Err(_serde::de::Error::invalid_length(0, &self))
                        }
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["kind", "payload"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "AnalyticsActionSource",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<AnalyticsActionSource>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};

const _: () = {
    extern crate serde as _serde;
    use tsify::Tsify;
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe,
        prelude::*,
    };

    ///
    #[repr(transparent)]
    pub struct JsType {
        obj: wasm_bindgen::JsValue,
    }

    const _: () = {
        use wasm_bindgen::__rt::core;
        use wasm_bindgen::convert::TryFromJsValue;
        use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
        use wasm_bindgen::convert::{LongRefFromWasmAbi, RefFromWasmAbi};
        use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
        use wasm_bindgen::describe::WasmDescribe;
        use wasm_bindgen::{JsCast, JsObject, JsValue};
        impl WasmDescribe for JsType {
            fn describe() {
                use wasm_bindgen::describe::*;
                inform(NAMED_EXTERNREF);
                inform(21u32);
                inform(65u32);
                inform(110u32);
                inform(97u32);
                inform(108u32);
                inform(121u32);
                inform(116u32);
                inform(105u32);
                inform(99u32);
                inform(115u32);
                inform(65u32);
                inform(99u32);
                inform(116u32);
                inform(105u32);
                inform(111u32);
                inform(110u32);
                inform(83u32);
                inform(111u32);
                inform(117u32);
                inform(114u32);
                inform(99u32);
                inform(101u32);
            }
        }
        impl IntoWasmAbi for JsType {
            type Abi = <JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.obj.into_abi()
            }
        }
        impl OptionIntoWasmAbi for JsType {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl<'a> OptionIntoWasmAbi for &'a JsType {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl FromWasmAbi for JsType {
            type Abi = <JsValue as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                JsType {
                    obj: JsValue::from_abi(js).into(),
                }
            }
        }
        impl OptionFromWasmAbi for JsType {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                *abi == 0
            }
        }
        impl<'a> IntoWasmAbi for &'a JsType {
            type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                (&self.obj).into_abi()
            }
        }
        impl RefFromWasmAbi for JsType {
            type Abi = <JsValue as RefFromWasmAbi>::Abi;
            type Anchor = core::mem::ManuallyDrop<JsType>;
            #[inline]
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                core::mem::ManuallyDrop::new(JsType {
                    obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
                })
            }
        }
        impl LongRefFromWasmAbi for JsType {
            type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
            type Anchor = JsType;
            #[inline]
            unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                JsType { obj: tmp.into() }
            }
        }
        impl From<JsValue> for JsType {
            #[inline]
            fn from(obj: JsValue) -> JsType {
                JsType { obj: obj.into() }
            }
        }
        impl AsRef<JsValue> for JsType {
            #[inline]
            fn as_ref(&self) -> &JsValue {
                self.obj.as_ref()
            }
        }
        impl AsRef<JsType> for JsType {
            #[inline]
            fn as_ref(&self) -> &JsType {
                self
            }
        }
        impl From<JsType> for JsValue {
            #[inline]
            fn from(obj: JsType) -> JsValue {
                obj.obj.into()
            }
        }
        impl JsCast for JsType {
            fn instanceof(val: &JsValue) -> bool {
                #[link(wasm_import_module = "__wbindgen_placeholder__")]
                #[cfg(all(
                    target_arch = "wasm32",
                    not(any(target_os = "emscripten", target_os = "wasi"))
                ))]
                extern "C" {
                    fn __wbg_instanceof_JsType_8d5c84bc1ae51d1b(val: u32) -> u32;
                }
                unsafe {
                    let idx = val.into_abi();
                    __wbg_instanceof_JsType_8d5c84bc1ae51d1b(idx) != 0
                }
            }
            #[inline]
            fn unchecked_from_js(val: JsValue) -> Self {
                JsType { obj: val.into() }
            }
            #[inline]
            fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                unsafe { &*(val as *const JsValue as *const JsType) }
            }
        }
        impl JsObject for JsType {}
    };

    impl core::ops::Deref for JsType {
        type Target = wasm_bindgen::JsValue;
        #[inline]
        fn deref(&self) -> &wasm_bindgen::JsValue {
            &self.obj
        }
    }
    #[cfg(target_arch = "wasm32")]

    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 148usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}^\x00\x00\x00\x00\x00\x01\x00\x00\x02\x06JsType(__wbg_instanceof_JsType_8d5c84bc1ae51d1b\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
    };
    impl Tsify for AnalyticsActionSource {
        type JsType = JsType;
        const DECL: &'static str = "export type AnalyticsActionSource = { kind: \"unknown\" } | { kind: \"asr\" } | { kind: \"navigation\" } | { kind: \"search\"; payload: { search_string: string } };";
    }
    #[cfg(target_arch = "wasm32")]

    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 254usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}\xc8\x00\x00\x00\x00\x00\x00\x00\x01\x9c\x01export type AnalyticsActionSource = { kind: \"unknown\" } | { kind: \"asr\" } | { kind: \"navigation\" } | { kind: \"search\"; payload: { search_string: string } };\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
    };
    impl WasmDescribe for AnalyticsActionSource {
        #[inline]
        fn describe() {
            <Self as Tsify>::JsType::describe()
        }
    }
    impl FromWasmAbi for AnalyticsActionSource
    where
        Self: _serde::de::DeserializeOwned,
    {
        type Abi = <JsType as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let result = Self::from_js(&JsType::from_abi(js));
            if let Err(err) = result {
                wasm_bindgen::throw_str(err.to_string().as_ref());
            }
            result.unwrap_throw()
        }
    }
    impl OptionFromWasmAbi for AnalyticsActionSource
    where
        Self: _serde::de::DeserializeOwned,
    {
        #[inline]
        fn is_none(js: &Self::Abi) -> bool {
            <JsType as OptionFromWasmAbi>::is_none(js)
        }
    }
};
pub struct Wid {
    pub data: u128,
}

const _: () = {
    extern crate serde as _serde;
    use tsify::Tsify;
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe,
        prelude::*,
    };

    ///
    #[repr(transparent)]
    pub struct JsType {
        obj: wasm_bindgen::JsValue,
    }

    const _: () = {
        use wasm_bindgen::__rt::core;
        use wasm_bindgen::convert::TryFromJsValue;
        use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
        use wasm_bindgen::convert::{LongRefFromWasmAbi, RefFromWasmAbi};
        use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
        use wasm_bindgen::describe::WasmDescribe;
        use wasm_bindgen::{JsCast, JsObject, JsValue};
        impl WasmDescribe for JsType {
            fn describe() {
                use wasm_bindgen::describe::*;
                inform(NAMED_EXTERNREF);
                inform(3u32);
                inform(87u32);
                inform(105u32);
                inform(100u32);
            }
        }
        impl IntoWasmAbi for JsType {
            type Abi = <JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.obj.into_abi()
            }
        }
        impl OptionIntoWasmAbi for JsType {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl<'a> OptionIntoWasmAbi for &'a JsType {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl FromWasmAbi for JsType {
            type Abi = <JsValue as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                JsType {
                    obj: JsValue::from_abi(js).into(),
                }
            }
        }
        impl OptionFromWasmAbi for JsType {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                *abi == 0
            }
        }
        impl<'a> IntoWasmAbi for &'a JsType {
            type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                (&self.obj).into_abi()
            }
        }
        impl RefFromWasmAbi for JsType {
            type Abi = <JsValue as RefFromWasmAbi>::Abi;
            type Anchor = core::mem::ManuallyDrop<JsType>;
            #[inline]
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                core::mem::ManuallyDrop::new(JsType {
                    obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
                })
            }
        }
        impl LongRefFromWasmAbi for JsType {
            type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
            type Anchor = JsType;
            #[inline]
            unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                JsType { obj: tmp.into() }
            }
        }
        impl From<JsValue> for JsType {
            #[inline]
            fn from(obj: JsValue) -> JsType {
                JsType { obj: obj.into() }
            }
        }
        impl AsRef<JsValue> for JsType {
            #[inline]
            fn as_ref(&self) -> &JsValue {
                self.obj.as_ref()
            }
        }
        impl AsRef<JsType> for JsType {
            #[inline]
            fn as_ref(&self) -> &JsType {
                self
            }
        }
        impl From<JsType> for JsValue {
            #[inline]
            fn from(obj: JsType) -> JsValue {
                obj.obj.into()
            }
        }
        impl JsCast for JsType {
            fn instanceof(val: &JsValue) -> bool {
                #[link(wasm_import_module = "__wbindgen_placeholder__")]
                #[cfg(all(
                    target_arch = "wasm32",
                    not(any(target_os = "emscripten", target_os = "wasi"))
                ))]
                extern "C" {
                    fn __wbg_instanceof_JsType_8d5c84bc1ae51d1b(val: u32) -> u32;
                }
                unsafe {
                    let idx = val.into_abi();
                    __wbg_instanceof_JsType_8d5c84bc1ae51d1b(idx) != 0
                }
            }
            #[inline]
            fn unchecked_from_js(val: JsValue) -> Self {
                JsType { obj: val.into() }
            }
            #[inline]
            fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                unsafe { &*(val as *const JsValue as *const JsType) }
            }
        }
        impl JsObject for JsType {}
    };

    impl core::ops::Deref for JsType {
        type Target = wasm_bindgen::JsValue;
        #[inline]
        fn deref(&self) -> &wasm_bindgen::JsValue {
            &self.obj
        }
    }
    #[cfg(target_arch = "wasm32")]

    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 148usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}^\x00\x00\x00\x00\x00\x01\x00\x00\x02\x06JsType(__wbg_instanceof_JsType_8d5c84bc1ae51d1b\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
    };
    impl Tsify for Wid {
        type JsType = JsType;
        const DECL: &'static str = "export interface Wid {\n    data: number;\n}";
    }
    #[cfg(target_arch = "wasm32")]

    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 139usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}U\x00\x00\x00\x00\x00\x00\x00\x01*export interface Wid {\n    data: number;\n}\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
    };
    impl WasmDescribe for Wid {
        #[inline]
        fn describe() {
            <Self as Tsify>::JsType::describe()
        }
    }
    impl IntoWasmAbi for Wid
    where
        Self: _serde::Serialize,
    {
        type Abi = <JsType as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.into_js().unwrap_throw().into_abi()
        }
    }
    impl OptionIntoWasmAbi for Wid
    where
        Self: _serde::Serialize,
    {
        #[inline]
        fn none() -> Self::Abi {
            <JsType as OptionIntoWasmAbi>::none()
        }
    }
    impl FromWasmAbi for Wid
    where
        Self: _serde::de::DeserializeOwned,
    {
        type Abi = <JsType as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let result = Self::from_js(&JsType::from_abi(js));
            console_log!("{}", js);
            console_log!("from_abi");
            if let Err(err) = result {
                console_log!("from_abi errored");
                wasm_bindgen::throw_str(err.to_string().as_ref());
                console_log!("from_abi errored");
            }
            result.unwrap_throw()
        }
    }
    impl OptionFromWasmAbi for Wid
    where
        Self: _serde::de::DeserializeOwned,
    {
        #[inline]
        fn is_none(js: &Self::Abi) -> bool {
            <JsType as OptionFromWasmAbi>::is_none(js)
        }
    }
};

impl ::core::marker::Copy for Wid {}

impl ::core::marker::StructuralPartialEq for Wid {}

impl ::core::cmp::PartialEq for Wid {
    #[inline]
    fn eq(&self, other: &Wid) -> bool {
        self.data == other.data
    }
}

impl ::core::marker::StructuralEq for Wid {}

impl ::core::cmp::Eq for Wid {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u128>;
    }
}

impl ::core::hash::Hash for Wid {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.data, state)
    }
}

impl ::core::cmp::PartialOrd for Wid {
    #[inline]
    fn partial_cmp(&self, other: &Wid) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.data, &other.data)
    }
}

impl ::core::cmp::Ord for Wid {
    #[inline]
    fn cmp(&self, other: &Wid) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.data, &other.data)
    }
}

impl ::core::fmt::Debug for Wid {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "Wid", "data", &&self.data)
    }
}

impl ::core::clone::Clone for Wid {
    #[inline]
    fn clone(&self) -> Wid {
        let _: ::core::clone::AssertParamIsClone<u128>;
        *self
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;

    impl _serde::Serialize for Wid {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state =
                _serde::Serializer::serialize_struct(__serializer, "Wid", false as usize + 1)?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "data", &self.data)?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;

    impl<'de> _serde::Deserialize<'de> for Wid {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "data" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"data" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Wid>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Wid;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Wid")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<u128>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct Wid with 1 element",
                            ));
                        }
                    };
                    _serde::__private::Ok(Wid { data: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                    );
                                }
                                __field0 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        u128,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("data")?,
                    };
                    _serde::__private::Ok(Wid { data: __field0 })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["data"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Wid",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Wid>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
use wasm_bindgen::JsValue;
pub type JsResult<T> = Result<T, JSError>;
pub trait ToJSResultTrait<T>: Sized {
    fn to_js_result(self) -> JsResult<T>;
    fn to_js_result_msg(self, msg: &str) -> JsResult<T> {
        match self.to_js_result() {
            Ok(value) => Ok(value),
            Err(err) => Err(JSError {
                message: {
                    let res = format!("{0}: {1}", msg, err.message);
                    res
                },
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
pub struct JSError {
    pub message: String,
}

impl ::core::fmt::Debug for JSError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "JSError", "message", &&self.message)
    }
}

impl ::core::clone::Clone for JSError {
    #[inline]
    fn clone(&self) -> JSError {
        JSError {
            message: ::core::clone::Clone::clone(&self.message),
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;

    impl _serde::Serialize for JSError {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state =
                _serde::Serializer::serialize_struct(__serializer, "JSError", false as usize + 1)?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "message",
                &self.message,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;

    impl<'de> _serde::Deserialize<'de> for JSError {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "message" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"message" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<JSError>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = JSError;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct JSError")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<String>(&mut __seq)?
                    {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct JSError with 1 element",
                            ));
                        }
                    };
                    _serde::__private::Ok(JSError { message: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "message",
                                        ),
                                    );
                                }
                                __field0 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        String,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("message")?,
                    };
                    _serde::__private::Ok(JSError { message: __field0 })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["message"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "JSError",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<JSError>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};

const _: () = {
    extern crate serde as _serde;
    use tsify::Tsify;
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe,
        prelude::*,
    };

    ///
    #[repr(transparent)]
    pub struct JsType {
        obj: wasm_bindgen::JsValue,
    }

    const _: () = {
        use wasm_bindgen::__rt::core;
        use wasm_bindgen::convert::TryFromJsValue;
        use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
        use wasm_bindgen::convert::{LongRefFromWasmAbi, RefFromWasmAbi};
        use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
        use wasm_bindgen::describe::WasmDescribe;
        use wasm_bindgen::{JsCast, JsObject, JsValue};
        impl WasmDescribe for JsType {
            fn describe() {
                use wasm_bindgen::describe::*;
                inform(NAMED_EXTERNREF);
                inform(7u32);
                inform(74u32);
                inform(83u32);
                inform(69u32);
                inform(114u32);
                inform(114u32);
                inform(111u32);
                inform(114u32);
            }
        }
        impl IntoWasmAbi for JsType {
            type Abi = <JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.obj.into_abi()
            }
        }
        impl OptionIntoWasmAbi for JsType {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl<'a> OptionIntoWasmAbi for &'a JsType {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl FromWasmAbi for JsType {
            type Abi = <JsValue as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                JsType {
                    obj: JsValue::from_abi(js).into(),
                }
            }
        }
        impl OptionFromWasmAbi for JsType {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                *abi == 0
            }
        }
        impl<'a> IntoWasmAbi for &'a JsType {
            type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                (&self.obj).into_abi()
            }
        }
        impl RefFromWasmAbi for JsType {
            type Abi = <JsValue as RefFromWasmAbi>::Abi;
            type Anchor = core::mem::ManuallyDrop<JsType>;
            #[inline]
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                core::mem::ManuallyDrop::new(JsType {
                    obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
                })
            }
        }
        impl LongRefFromWasmAbi for JsType {
            type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
            type Anchor = JsType;
            #[inline]
            unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                JsType { obj: tmp.into() }
            }
        }
        impl From<JsValue> for JsType {
            #[inline]
            fn from(obj: JsValue) -> JsType {
                JsType { obj: obj.into() }
            }
        }
        impl AsRef<JsValue> for JsType {
            #[inline]
            fn as_ref(&self) -> &JsValue {
                self.obj.as_ref()
            }
        }
        impl AsRef<JsType> for JsType {
            #[inline]
            fn as_ref(&self) -> &JsType {
                self
            }
        }
        impl From<JsType> for JsValue {
            #[inline]
            fn from(obj: JsType) -> JsValue {
                obj.obj.into()
            }
        }
        impl JsCast for JsType {
            fn instanceof(val: &JsValue) -> bool {
                #[link(wasm_import_module = "__wbindgen_placeholder__")]
                #[cfg(all(
                    target_arch = "wasm32",
                    not(any(target_os = "emscripten", target_os = "wasi"))
                ))]
                extern "C" {
                    fn __wbg_instanceof_JsType_8d5c84bc1ae51d1b(val: u32) -> u32;
                }
                unsafe {
                    let idx = val.into_abi();
                    __wbg_instanceof_JsType_8d5c84bc1ae51d1b(idx) != 0
                }
            }
            #[inline]
            fn unchecked_from_js(val: JsValue) -> Self {
                JsType { obj: val.into() }
            }
            #[inline]
            fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                unsafe { &*(val as *const JsValue as *const JsType) }
            }
        }
        impl JsObject for JsType {}
    };

    impl core::ops::Deref for JsType {
        type Target = wasm_bindgen::JsValue;
        #[inline]
        fn deref(&self) -> &wasm_bindgen::JsValue {
            &self.obj
        }
    }
    #[cfg(target_arch = "wasm32")]

    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 148usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}^\x00\x00\x00\x00\x00\x01\x00\x00\x02\x06JsType(__wbg_instanceof_JsType_8d5c84bc1ae51d1b\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
    };
    impl Tsify for JSError {
        type JsType = JsType;
        const DECL: &'static str = "export interface JSError {\n    message: string;\n}";
    }
    #[cfg(target_arch = "wasm32")]

    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 146usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}\\\x00\x00\x00\x00\x00\x00\x00\x011export interface JSError {\n    message: string;\n}\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
    };
    impl WasmDescribe for JSError {
        #[inline]
        fn describe() {
            <Self as Tsify>::JsType::describe()
        }
    }
    impl FromWasmAbi for JSError
    where
        Self: _serde::de::DeserializeOwned,
    {
        type Abi = <JsType as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let result = Self::from_js(&JsType::from_abi(js));
            if let Err(err) = result {
                wasm_bindgen::throw_str(err.to_string().as_ref());
            }
            result.unwrap_throw()
        }
    }
    impl OptionFromWasmAbi for JSError
    where
        Self: _serde::de::DeserializeOwned,
    {
        #[inline]
        fn is_none(js: &Self::Abi) -> bool {
            <JsType as OptionFromWasmAbi>::is_none(js)
        }
    }
};
pub struct JSAnalyticItem {
    pub order_id: String,
    pub action_type: String,
    pub test_id: Option<i16>,
    pub object: Option<String>,
}

impl ::core::fmt::Debug for JSAnalyticItem {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "JSAnalyticItem",
            "order_id",
            &self.order_id,
            "action_type",
            &self.action_type,
            "test_id",
            &self.test_id,
            "object",
            &&self.object,
        )
    }
}

impl ::core::clone::Clone for JSAnalyticItem {
    #[inline]
    fn clone(&self) -> JSAnalyticItem {
        JSAnalyticItem {
            order_id: ::core::clone::Clone::clone(&self.order_id),
            action_type: ::core::clone::Clone::clone(&self.action_type),
            test_id: ::core::clone::Clone::clone(&self.test_id),
            object: ::core::clone::Clone::clone(&self.object),
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;

    impl _serde::Serialize for JSAnalyticItem {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "JSAnalyticItem",
                false as usize + 1 + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "order_id",
                &self.order_id,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "action_type",
                &self.action_type,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "test_id",
                &self.test_id,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "object",
                &self.object,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;

    impl<'de> _serde::Deserialize<'de> for JSAnalyticItem {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "order_id" => _serde::__private::Ok(__Field::__field0),
                        "action_type" => _serde::__private::Ok(__Field::__field1),
                        "test_id" => _serde::__private::Ok(__Field::__field2),
                        "object" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"order_id" => _serde::__private::Ok(__Field::__field0),
                        b"action_type" => _serde::__private::Ok(__Field::__field1),
                        b"test_id" => _serde::__private::Ok(__Field::__field2),
                        b"object" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<JSAnalyticItem>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = JSAnalyticItem;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct JSAnalyticItem")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<String>(&mut __seq)?
                    {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct JSAnalyticItem with 4 elements",
                            ));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<String>(&mut __seq)?
                    {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct JSAnalyticItem with 4 elements",
                            ));
                        }
                    };
                    let __field2 =
                        match _serde::de::SeqAccess::next_element::<Option<i16>>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct JSAnalyticItem with 4 elements",
                                ));
                            }
                        };
                    let __field3 =
                        match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct JSAnalyticItem with 4 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(JSAnalyticItem {
                        order_id: __field0,
                        action_type: __field1,
                        test_id: __field2,
                        object: __field3,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<Option<i16>> =
                        _serde::__private::None;
                    let mut __field3: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "order_id",
                                        ),
                                    );
                                }
                                __field0 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        String,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "action_type",
                                        ),
                                    );
                                }
                                __field1 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        String,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "test_id",
                                        ),
                                    );
                                }
                                __field2 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        Option<i16>,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "object",
                                        ),
                                    );
                                }
                                __field3 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        Option<String>,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("order_id")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("action_type")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => _serde::__private::de::missing_field("test_id")?,
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => _serde::__private::de::missing_field("object")?,
                    };
                    _serde::__private::Ok(JSAnalyticItem {
                        order_id: __field0,
                        action_type: __field1,
                        test_id: __field2,
                        object: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] =
                &["order_id", "action_type", "test_id", "object"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "JSAnalyticItem",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<JSAnalyticItem>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};

const _: () = {
    extern crate serde as _serde;
    use tsify::Tsify;
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe,
        prelude::*,
    };

    ///
    #[repr(transparent)]
    pub struct JsType {
        obj: wasm_bindgen::JsValue,
    }

    const _: () = {
        use wasm_bindgen::__rt::core;
        use wasm_bindgen::convert::TryFromJsValue;
        use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
        use wasm_bindgen::convert::{LongRefFromWasmAbi, RefFromWasmAbi};
        use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
        use wasm_bindgen::describe::WasmDescribe;
        use wasm_bindgen::{JsCast, JsObject, JsValue};
        impl WasmDescribe for JsType {
            fn describe() {
                use wasm_bindgen::describe::*;
                inform(NAMED_EXTERNREF);
                inform(14u32);
                inform(74u32);
                inform(83u32);
                inform(65u32);
                inform(110u32);
                inform(97u32);
                inform(108u32);
                inform(121u32);
                inform(116u32);
                inform(105u32);
                inform(99u32);
                inform(73u32);
                inform(116u32);
                inform(101u32);
                inform(109u32);
            }
        }
        impl IntoWasmAbi for JsType {
            type Abi = <JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.obj.into_abi()
            }
        }
        impl OptionIntoWasmAbi for JsType {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl<'a> OptionIntoWasmAbi for &'a JsType {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl FromWasmAbi for JsType {
            type Abi = <JsValue as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                JsType {
                    obj: JsValue::from_abi(js).into(),
                }
            }
        }
        impl OptionFromWasmAbi for JsType {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                *abi == 0
            }
        }
        impl<'a> IntoWasmAbi for &'a JsType {
            type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                (&self.obj).into_abi()
            }
        }
        impl RefFromWasmAbi for JsType {
            type Abi = <JsValue as RefFromWasmAbi>::Abi;
            type Anchor = core::mem::ManuallyDrop<JsType>;
            #[inline]
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                core::mem::ManuallyDrop::new(JsType {
                    obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
                })
            }
        }
        impl LongRefFromWasmAbi for JsType {
            type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
            type Anchor = JsType;
            #[inline]
            unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                JsType { obj: tmp.into() }
            }
        }
        impl From<JsValue> for JsType {
            #[inline]
            fn from(obj: JsValue) -> JsType {
                JsType { obj: obj.into() }
            }
        }
        impl AsRef<JsValue> for JsType {
            #[inline]
            fn as_ref(&self) -> &JsValue {
                self.obj.as_ref()
            }
        }
        impl AsRef<JsType> for JsType {
            #[inline]
            fn as_ref(&self) -> &JsType {
                self
            }
        }
        impl From<JsType> for JsValue {
            #[inline]
            fn from(obj: JsType) -> JsValue {
                obj.obj.into()
            }
        }
        impl JsCast for JsType {
            fn instanceof(val: &JsValue) -> bool {
                #[link(wasm_import_module = "__wbindgen_placeholder__")]
                #[cfg(all(
                    target_arch = "wasm32",
                    not(any(target_os = "emscripten", target_os = "wasi"))
                ))]
                extern "C" {
                    fn __wbg_instanceof_JsType_8d5c84bc1ae51d1b(val: u32) -> u32;
                }
                unsafe {
                    let idx = val.into_abi();
                    __wbg_instanceof_JsType_8d5c84bc1ae51d1b(idx) != 0
                }
            }
            #[inline]
            fn unchecked_from_js(val: JsValue) -> Self {
                JsType { obj: val.into() }
            }
            #[inline]
            fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                unsafe { &*(val as *const JsValue as *const JsType) }
            }
        }
        impl JsObject for JsType {}
    };

    impl core::ops::Deref for JsType {
        type Target = wasm_bindgen::JsValue;
        #[inline]
        fn deref(&self) -> &wasm_bindgen::JsValue {
            &self.obj
        }
    }
    #[cfg(target_arch = "wasm32")]

    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 148usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}^\x00\x00\x00\x00\x00\x01\x00\x00\x02\x06JsType(__wbg_instanceof_JsType_8d5c84bc1ae51d1b\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
    };
    impl Tsify for JSAnalyticItem {
        type JsType = JsType;
        const DECL: &'static str = "export interface JSAnalyticItem {\n    order_id: string;\n    action_type: string;\n    test_id: number | null;\n    object: string | null;\n}";
    }
    #[cfg(target_arch = "wasm32")]

    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 235usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}\xb5\x00\x00\x00\x00\x00\x00\x00\x01\x89\x01export interface JSAnalyticItem {\n    order_id: string;\n    action_type: string;\n    test_id: number | null;\n    object: string | null;\n}\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
    };
    impl WasmDescribe for JSAnalyticItem {
        #[inline]
        fn describe() {
            <Self as Tsify>::JsType::describe()
        }
    }
    impl FromWasmAbi for JSAnalyticItem
    where
        Self: _serde::de::DeserializeOwned,
    {
        type Abi = <JsType as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let result = Self::from_js(&JsType::from_abi(js));
            if let Err(err) = result {
                wasm_bindgen::throw_str(err.to_string().as_ref());
            }
            result.unwrap_throw()
        }
    }
    impl OptionFromWasmAbi for JSAnalyticItem
    where
        Self: _serde::de::DeserializeOwned,
    {
        #[inline]
        fn is_none(js: &Self::Abi) -> bool {
            <JsType as OptionFromWasmAbi>::is_none(js)
        }
    }
};
pub enum JSStreamStatus {
    Start,
    Retry(i8),
    GiveUp,
    Error(String),
}

impl ::core::fmt::Debug for JSStreamStatus {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            JSStreamStatus::Start => ::core::fmt::Formatter::write_str(f, "Start"),
            JSStreamStatus::Retry(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Retry", &__self_0)
            }
            JSStreamStatus::GiveUp => ::core::fmt::Formatter::write_str(f, "GiveUp"),
            JSStreamStatus::Error(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Error", &__self_0)
            }
        }
    }
}

impl ::core::clone::Clone for JSStreamStatus {
    #[inline]
    fn clone(&self) -> JSStreamStatus {
        match self {
            JSStreamStatus::Start => JSStreamStatus::Start,
            JSStreamStatus::Retry(__self_0) => {
                JSStreamStatus::Retry(::core::clone::Clone::clone(__self_0))
            }
            JSStreamStatus::GiveUp => JSStreamStatus::GiveUp,
            JSStreamStatus::Error(__self_0) => {
                JSStreamStatus::Error(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;

    impl _serde::Serialize for JSStreamStatus {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                JSStreamStatus::Start => {
                    let mut __struct =
                        _serde::Serializer::serialize_struct(__serializer, "JSStreamStatus", 1)?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "kind",
                        &_serde::__private::ser::AdjacentlyTaggedEnumVariant {
                            enum_name: "JSStreamStatus",
                            variant_index: 0u32,
                            variant_name: "start",
                        },
                    )?;
                    _serde::ser::SerializeStruct::end(__struct)
                }
                JSStreamStatus::Retry(ref __field0) => {
                    let mut __struct =
                        _serde::Serializer::serialize_struct(__serializer, "JSStreamStatus", 2)?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "kind",
                        &_serde::__private::ser::AdjacentlyTaggedEnumVariant {
                            enum_name: "JSStreamStatus",
                            variant_index: 1u32,
                            variant_name: "retry",
                        },
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "payload",
                        __field0,
                    )?;
                    _serde::ser::SerializeStruct::end(__struct)
                }
                JSStreamStatus::GiveUp => {
                    let mut __struct =
                        _serde::Serializer::serialize_struct(__serializer, "JSStreamStatus", 1)?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "kind",
                        &_serde::__private::ser::AdjacentlyTaggedEnumVariant {
                            enum_name: "JSStreamStatus",
                            variant_index: 2u32,
                            variant_name: "give_up",
                        },
                    )?;
                    _serde::ser::SerializeStruct::end(__struct)
                }
                JSStreamStatus::Error(ref __field0) => {
                    let mut __struct =
                        _serde::Serializer::serialize_struct(__serializer, "JSStreamStatus", 2)?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "kind",
                        &_serde::__private::ser::AdjacentlyTaggedEnumVariant {
                            enum_name: "JSStreamStatus",
                            variant_index: 3u32,
                            variant_name: "error",
                        },
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "payload",
                        __field0,
                    )?;
                    _serde::ser::SerializeStruct::end(__struct)
                }
            }
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;

    impl<'de> _serde::Deserialize<'de> for JSStreamStatus {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"variant index 0 <= i < 4",
                        )),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "start" => _serde::__private::Ok(__Field::__field0),
                        "retry" => _serde::__private::Ok(__Field::__field1),
                        "give_up" => _serde::__private::Ok(__Field::__field2),
                        "error" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                            __value, VARIANTS,
                        )),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"start" => _serde::__private::Ok(__Field::__field0),
                        b"retry" => _serde::__private::Ok(__Field::__field1),
                        b"give_up" => _serde::__private::Ok(__Field::__field2),
                        b"error" => _serde::__private::Ok(__Field::__field3),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            ))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["start", "retry", "give_up", "error"];
            #[doc(hidden)]
            struct __Seed<'de> {
                field: __Field,
                marker: _serde::__private::PhantomData<JSStreamStatus>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::DeserializeSeed<'de> for __Seed<'de> {
                type Value = JSStreamStatus;
                fn deserialize<__D>(
                    self,
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self::Value, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    match self.field {
                        __Field::__field0 => {
                            match _serde::Deserializer::deserialize_any(
                                __deserializer,
                                _serde::__private::de::UntaggedUnitVisitor::new(
                                    "JSStreamStatus",
                                    "Start",
                                ),
                            ) {
                                _serde::__private::Ok(()) => {
                                    _serde::__private::Ok(JSStreamStatus::Start)
                                }
                                _serde::__private::Err(__err) => _serde::__private::Err(__err),
                            }
                        }
                        __Field::__field1 => _serde::__private::Result::map(
                            <i8 as _serde::Deserialize>::deserialize(__deserializer),
                            JSStreamStatus::Retry,
                        ),
                        __Field::__field2 => {
                            match _serde::Deserializer::deserialize_any(
                                __deserializer,
                                _serde::__private::de::UntaggedUnitVisitor::new(
                                    "JSStreamStatus",
                                    "GiveUp",
                                ),
                            ) {
                                _serde::__private::Ok(()) => {
                                    _serde::__private::Ok(JSStreamStatus::GiveUp)
                                }
                                _serde::__private::Err(__err) => _serde::__private::Err(__err),
                            }
                        }
                        __Field::__field3 => _serde::__private::Result::map(
                            <String as _serde::Deserialize>::deserialize(__deserializer),
                            JSStreamStatus::Error,
                        ),
                    }
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<JSStreamStatus>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = JSStreamStatus;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "adjacently tagged enum JSStreamStatus",
                    )
                }
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    match {
                        let mut __rk: _serde::__private::Option<
                            _serde::__private::de::TagOrContentField,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__k) =
                            _serde::de::MapAccess::next_key_seed(
                                &mut __map,
                                _serde::__private::de::TagContentOtherFieldVisitor {
                                    tag: "kind",
                                    content: "payload",
                                },
                            )?
                        {
                            match __k {
                                _serde::__private::de::TagContentOtherField::Other => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                    continue;
                                }
                                _serde::__private::de::TagContentOtherField::Tag => {
                                    __rk = _serde::__private::Some(
                                        _serde::__private::de::TagOrContentField::Tag,
                                    );
                                    break;
                                }
                                _serde::__private::de::TagContentOtherField::Content => {
                                    __rk = _serde::__private::Some(
                                        _serde::__private::de::TagOrContentField::Content,
                                    );
                                    break;
                                }
                            }
                        }
                        __rk
                    } {
                        _serde::__private::Some(_serde::__private::de::TagOrContentField::Tag) => {
                            let __field = _serde::de::MapAccess::next_value_seed(
                                &mut __map,
                                _serde::__private::de::AdjacentlyTaggedEnumVariantSeed::<__Field> {
                                    enum_name: "JSStreamStatus",
                                    variants: VARIANTS,
                                    fields_enum: _serde::__private::PhantomData,
                                },
                            )?;
                            match {
                                let mut __rk: _serde::__private::Option<
                                    _serde::__private::de::TagOrContentField,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__k) =
                                    _serde::de::MapAccess::next_key_seed(
                                        &mut __map,
                                        _serde::__private::de::TagContentOtherFieldVisitor {
                                            tag: "kind",
                                            content: "payload",
                                        },
                                    )?
                                {
                                    match __k {
                                        _serde::__private::de::TagContentOtherField::Other => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                            continue;
                                        }
                                        _serde::__private::de::TagContentOtherField::Tag => {
                                            __rk = _serde::__private::Some(
                                                _serde::__private::de::TagOrContentField::Tag,
                                            );
                                            break;
                                        }
                                        _serde::__private::de::TagContentOtherField::Content => {
                                            __rk = _serde::__private::Some(
                                                _serde::__private::de::TagOrContentField::Content,
                                            );
                                            break;
                                        }
                                    }
                                }
                                __rk
                            } {
                                _serde::__private::Some(
                                    _serde::__private::de::TagOrContentField::Tag,
                                ) => _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::duplicate_field("kind"),
                                ),
                                _serde::__private::Some(
                                    _serde::__private::de::TagOrContentField::Content,
                                ) => {
                                    let __ret = _serde::de::MapAccess::next_value_seed(
                                        &mut __map,
                                        __Seed {
                                            field: __field,
                                            marker: _serde::__private::PhantomData,
                                            lifetime: _serde::__private::PhantomData,
                                        },
                                    )?;
                                    match {
                                        let mut __rk: _serde::__private::Option<
                                            _serde::__private::de::TagOrContentField,
                                        > = _serde::__private::None;
                                        while let _serde::__private::Some(__k) = _serde::de::MapAccess::next_key_seed(
                                            &mut __map,
                                            _serde::__private::de::TagContentOtherFieldVisitor {
                                                tag: "kind",
                                                content: "payload",
                                            },
                                        )? {
                                            match __k {
                                                _serde::__private::de::TagContentOtherField::Other => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                    continue;
                                                }
                                                _serde::__private::de::TagContentOtherField::Tag => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Tag,
                                                    );
                                                    break;
                                                }
                                                _serde::__private::de::TagContentOtherField::Content => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Content,
                                                    );
                                                    break;
                                                }
                                            }
                                        }
                                        __rk
                                    } {
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Tag,
                                        ) => _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "kind",
                                            ),
                                        ),
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Content,
                                        ) => _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "payload",
                                            ),
                                        ),
                                        _serde::__private::None => _serde::__private::Ok(__ret),
                                    }
                                }
                                _serde::__private::None => match __field {
                                    __Field::__field0 => {
                                        _serde::__private::Ok(JSStreamStatus::Start)
                                    }
                                    __Field::__field1 => {
                                        _serde::__private::de::missing_field("payload")
                                            .map(JSStreamStatus::Retry)
                                    }
                                    __Field::__field2 => {
                                        _serde::__private::Ok(JSStreamStatus::GiveUp)
                                    }
                                    __Field::__field3 => {
                                        _serde::__private::de::missing_field("payload")
                                            .map(JSStreamStatus::Error)
                                    }
                                },
                            }
                        }
                        _serde::__private::Some(
                            _serde::__private::de::TagOrContentField::Content,
                        ) => {
                            let __content = _serde::de::MapAccess::next_value::<
                                _serde::__private::de::Content,
                            >(&mut __map)?;
                            match {
                                let mut __rk: _serde::__private::Option<
                                    _serde::__private::de::TagOrContentField,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__k) =
                                    _serde::de::MapAccess::next_key_seed(
                                        &mut __map,
                                        _serde::__private::de::TagContentOtherFieldVisitor {
                                            tag: "kind",
                                            content: "payload",
                                        },
                                    )?
                                {
                                    match __k {
                                        _serde::__private::de::TagContentOtherField::Other => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                            continue;
                                        }
                                        _serde::__private::de::TagContentOtherField::Tag => {
                                            __rk = _serde::__private::Some(
                                                _serde::__private::de::TagOrContentField::Tag,
                                            );
                                            break;
                                        }
                                        _serde::__private::de::TagContentOtherField::Content => {
                                            __rk = _serde::__private::Some(
                                                _serde::__private::de::TagOrContentField::Content,
                                            );
                                            break;
                                        }
                                    }
                                }
                                __rk
                            } {
                                _serde::__private::Some(
                                    _serde::__private::de::TagOrContentField::Tag,
                                ) => {
                                    let __deserializer = _serde::__private::de::ContentDeserializer::<
                                        __A::Error,
                                    >::new(
                                        __content
                                    );
                                    let __ret = match _serde::de::MapAccess::next_value_seed(
                                        &mut __map,
                                        _serde::__private::de::AdjacentlyTaggedEnumVariantSeed::<
                                            __Field,
                                        > {
                                            enum_name: "JSStreamStatus",
                                            variants: VARIANTS,
                                            fields_enum: _serde::__private::PhantomData,
                                        },
                                    )? {
                                        __Field::__field0 => {
                                            match _serde::Deserializer::deserialize_any(
                                                __deserializer,
                                                _serde::__private::de::UntaggedUnitVisitor::new(
                                                    "JSStreamStatus",
                                                    "Start",
                                                ),
                                            ) {
                                                _serde::__private::Ok(()) => {
                                                    _serde::__private::Ok(JSStreamStatus::Start)
                                                }
                                                _serde::__private::Err(__err) => {
                                                    _serde::__private::Err(__err)
                                                }
                                            }
                                        }
                                        __Field::__field1 => _serde::__private::Result::map(
                                            <i8 as _serde::Deserialize>::deserialize(
                                                __deserializer,
                                            ),
                                            JSStreamStatus::Retry,
                                        ),
                                        __Field::__field2 => {
                                            match _serde::Deserializer::deserialize_any(
                                                __deserializer,
                                                _serde::__private::de::UntaggedUnitVisitor::new(
                                                    "JSStreamStatus",
                                                    "GiveUp",
                                                ),
                                            ) {
                                                _serde::__private::Ok(()) => {
                                                    _serde::__private::Ok(JSStreamStatus::GiveUp)
                                                }
                                                _serde::__private::Err(__err) => {
                                                    _serde::__private::Err(__err)
                                                }
                                            }
                                        }
                                        __Field::__field3 => _serde::__private::Result::map(
                                            <String as _serde::Deserialize>::deserialize(
                                                __deserializer,
                                            ),
                                            JSStreamStatus::Error,
                                        ),
                                    }?;
                                    match {
                                        let mut __rk: _serde::__private::Option<
                                            _serde::__private::de::TagOrContentField,
                                        > = _serde::__private::None;
                                        while let _serde::__private::Some(__k) = _serde::de::MapAccess::next_key_seed(
                                            &mut __map,
                                            _serde::__private::de::TagContentOtherFieldVisitor {
                                                tag: "kind",
                                                content: "payload",
                                            },
                                        )? {
                                            match __k {
                                                _serde::__private::de::TagContentOtherField::Other => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                    continue;
                                                }
                                                _serde::__private::de::TagContentOtherField::Tag => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Tag,
                                                    );
                                                    break;
                                                }
                                                _serde::__private::de::TagContentOtherField::Content => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Content,
                                                    );
                                                    break;
                                                }
                                            }
                                        }
                                        __rk
                                    } {
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Tag,
                                        ) => _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "kind",
                                            ),
                                        ),
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Content,
                                        ) => _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "payload",
                                            ),
                                        ),
                                        _serde::__private::None => _serde::__private::Ok(__ret),
                                    }
                                }
                                _serde::__private::Some(
                                    _serde::__private::de::TagOrContentField::Content,
                                ) => _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::duplicate_field("payload"),
                                ),
                                _serde::__private::None => _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field("kind"),
                                ),
                            }
                        }
                        _serde::__private::None => _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::missing_field("kind"),
                        ),
                    }
                }
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    match _serde::de::SeqAccess::next_element(&mut __seq)? {
                        _serde::__private::Some(__field) => {
                            match _serde::de::SeqAccess::next_element_seed(
                                &mut __seq,
                                __Seed {
                                    field: __field,
                                    marker: _serde::__private::PhantomData,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )? {
                                _serde::__private::Some(__ret) => _serde::__private::Ok(__ret),
                                _serde::__private::None => _serde::__private::Err(
                                    _serde::de::Error::invalid_length(1, &self),
                                ),
                            }
                        }
                        _serde::__private::None => {
                            _serde::__private::Err(_serde::de::Error::invalid_length(0, &self))
                        }
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["kind", "payload"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "JSStreamStatus",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<JSStreamStatus>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};

const _: () = {
    extern crate serde as _serde;
    use tsify::Tsify;
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe,
        prelude::*,
    };

    ///
    #[repr(transparent)]
    pub struct JsType {
        obj: wasm_bindgen::JsValue,
    }

    const _: () = {
        use wasm_bindgen::__rt::core;
        use wasm_bindgen::convert::TryFromJsValue;
        use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
        use wasm_bindgen::convert::{LongRefFromWasmAbi, RefFromWasmAbi};
        use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
        use wasm_bindgen::describe::WasmDescribe;
        use wasm_bindgen::{JsCast, JsObject, JsValue};
        impl WasmDescribe for JsType {
            fn describe() {
                use wasm_bindgen::describe::*;
                inform(NAMED_EXTERNREF);
                inform(14u32);
                inform(74u32);
                inform(83u32);
                inform(83u32);
                inform(116u32);
                inform(114u32);
                inform(101u32);
                inform(97u32);
                inform(109u32);
                inform(83u32);
                inform(116u32);
                inform(97u32);
                inform(116u32);
                inform(117u32);
                inform(115u32);
            }
        }
        impl IntoWasmAbi for JsType {
            type Abi = <JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.obj.into_abi()
            }
        }
        impl OptionIntoWasmAbi for JsType {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl<'a> OptionIntoWasmAbi for &'a JsType {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl FromWasmAbi for JsType {
            type Abi = <JsValue as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                JsType {
                    obj: JsValue::from_abi(js).into(),
                }
            }
        }
        impl OptionFromWasmAbi for JsType {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                *abi == 0
            }
        }
        impl<'a> IntoWasmAbi for &'a JsType {
            type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                (&self.obj).into_abi()
            }
        }
        impl RefFromWasmAbi for JsType {
            type Abi = <JsValue as RefFromWasmAbi>::Abi;
            type Anchor = core::mem::ManuallyDrop<JsType>;
            #[inline]
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                core::mem::ManuallyDrop::new(JsType {
                    obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
                })
            }
        }
        impl LongRefFromWasmAbi for JsType {
            type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
            type Anchor = JsType;
            #[inline]
            unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                JsType { obj: tmp.into() }
            }
        }
        impl From<JsValue> for JsType {
            #[inline]
            fn from(obj: JsValue) -> JsType {
                JsType { obj: obj.into() }
            }
        }
        impl AsRef<JsValue> for JsType {
            #[inline]
            fn as_ref(&self) -> &JsValue {
                self.obj.as_ref()
            }
        }
        impl AsRef<JsType> for JsType {
            #[inline]
            fn as_ref(&self) -> &JsType {
                self
            }
        }
        impl From<JsType> for JsValue {
            #[inline]
            fn from(obj: JsType) -> JsValue {
                obj.obj.into()
            }
        }
        impl JsCast for JsType {
            fn instanceof(val: &JsValue) -> bool {
                #[link(wasm_import_module = "__wbindgen_placeholder__")]
                #[cfg(all(
                    target_arch = "wasm32",
                    not(any(target_os = "emscripten", target_os = "wasi"))
                ))]
                extern "C" {
                    fn __wbg_instanceof_JsType_8d5c84bc1ae51d1b(val: u32) -> u32;
                }
                unsafe {
                    let idx = val.into_abi();
                    __wbg_instanceof_JsType_8d5c84bc1ae51d1b(idx) != 0
                }
            }
            #[inline]
            fn unchecked_from_js(val: JsValue) -> Self {
                JsType { obj: val.into() }
            }
            #[inline]
            fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                unsafe { &*(val as *const JsValue as *const JsType) }
            }
        }
        impl JsObject for JsType {}
    };

    impl core::ops::Deref for JsType {
        type Target = wasm_bindgen::JsValue;
        #[inline]
        fn deref(&self) -> &wasm_bindgen::JsValue {
            &self.obj
        }
    }
    #[cfg(target_arch = "wasm32")]

    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 148usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}^\x00\x00\x00\x00\x00\x01\x00\x00\x02\x06JsType(__wbg_instanceof_JsType_8d5c84bc1ae51d1b\x00\x00\x00\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
    };
    impl Tsify for JSStreamStatus {
        type JsType = JsType;
        const DECL: &'static str = "export type JSStreamStatus = { kind: \"start\" } | { kind: \"retry\"; payload: number } | { kind: \"give_up\" } | { kind: \"error\"; payload: string };";
    }
    #[cfg(target_arch = "wasm32")]

    const _: () = {
        static _INCLUDED_FILES: &[&str] = &[];
        #[link_section = "__wasm_bindgen_unstable"]
        pub static _GENERATED: [u8; 241usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.90\"}\xbb\x00\x00\x00\x00\x00\x00\x00\x01\x8f\x01export type JSStreamStatus = { kind: \"start\" } | { kind: \"retry\"; payload: number } | { kind: \"give_up\" } | { kind: \"error\"; payload: string };\x00\x00 learn-rust-wasm-afb3a028510d0a73\x00\x00";
    };
    impl WasmDescribe for JSStreamStatus {
        #[inline]
        fn describe() {
            <Self as Tsify>::JsType::describe()
        }
    }
    impl FromWasmAbi for JSStreamStatus
    where
        Self: _serde::de::DeserializeOwned,
    {
        type Abi = <JsType as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let result = Self::from_js(&JsType::from_abi(js));
            if let Err(err) = result {
                wasm_bindgen::throw_str(err.to_string().as_ref());
            }
            result.unwrap_throw()
        }
    }
    impl OptionFromWasmAbi for JSStreamStatus
    where
        Self: _serde::de::DeserializeOwned,
    {
        #[inline]
        fn is_none(js: &Self::Abi) -> bool {
            <JsType as OptionFromWasmAbi>::is_none(js)
        }
    }
};
