#[cfg(not(any(feature = "frontend", feature = "backend")))]
compile_error!("one of features \"frontend\" or \"backend\" must be enabled");

#[cfg(all(feature = "frontend", feature = "backend"))]
compile_error!("features \"frontend\" and \"backend\" cannot be enabled at the same time");

#[cfg(feature = "frontend")]
use js_sys::{Function, Promise, Reflect};
#[cfg(feature = "frontend")]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(feature = "frontend")]
use wasm_bindgen_futures::JsFuture;

#[cfg(feature = "backend")]
pub use tauri;

#[derive(serde::Serialize)]
pub struct WrappedArgs<T>
where
    T: serde::ser::Serialize + ?Sized,
{
    args: T,
}

#[cfg(feature = "frontend")]
pub async fn tauri_command<S, R>(command: &str, args: &S) -> R
where
    S: serde::ser::Serialize + ?Sized,
    R: serde::de::DeserializeOwned + ?Sized,
{
    let tauri = web_sys::window().unwrap().get("__TAURI__").unwrap();
    let invoke = Reflect::get(&tauri.into(), &"invoke".into()).unwrap();
    let invoke_function = invoke.dyn_ref::<Function>().unwrap();

    let wrapped_args = WrappedArgs { args };
    let js_args = serde_wasm_bindgen::to_value(&wrapped_args).unwrap();

    let response = invoke_function
        .call2(&invoke_function, &command.into(), &js_args)
        .unwrap();
    let response_promise = response.dyn_into::<Promise>().unwrap();
    let response_future = JsFuture::from(response_promise);
    let command_res = response_future.await.unwrap();
    serde_wasm_bindgen::from_value(command_res).unwrap()
}

pub fn serialize<T>(value: &T) -> Result<String, serde_json::Error>
where
    T: serde::ser::Serialize + ?Sized,
{
    serde_json::to_string(value)
}

pub fn deserialize<T>(str_value: &str) -> Result<T, serde_json::Error>
where
    T: serde::de::DeserializeOwned + ?Sized,
{
    serde_json::from_str(str_value)
}

#[cfg(feature = "frontend")]
pub fn js_serialize<T>(value: &T) -> Result<JsValue, serde_wasm_bindgen::Error>
where
    T: serde::ser::Serialize + ?Sized,
{
    serde_wasm_bindgen::to_value(value)
}

#[cfg(feature = "frontend")]
pub fn js_deserialize<T>(js_value: JsValue) -> Result<T, serde_wasm_bindgen::Error>
where
    T: serde::de::DeserializeOwned + ?Sized,
{
    serde_wasm_bindgen::from_value(js_value)
}

#[macro_export]
macro_rules! serialize_args {
    ( $( $arg:expr ),* ) => {
        {
            let mut arg_map = ::std::collections::HashMap::<String, String>::new();
            $(
                arg_map.insert(stringify!($arg).to_owned(), ::macro_utils::serialize(&$arg).unwrap());
            )*
            arg_map
        }
    };
}

#[macro_export]
macro_rules! deserialize_args {
    ( $serialized_args:expr, $( $arg_name:ident: $arg_type:ty ),* ) => {
        $(
            let $arg_name: $arg_type = ::macro_utils::deserialize($serialized_args.get(stringify!($arg_name)).unwrap()).unwrap();
        )*
    };
}
