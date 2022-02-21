use std::path::PathBuf;

use red4ext_rs::prelude::*;
use toml::Value;

define_plugin! {
    name: "toml-reds",
    author: "jekky",
    version: 1:0:0,
    on_register: {
        register_function!("Toml.LoadConfig", load_value);
    }
}

fn load_value(name: String) -> Ref<RED4ext::IScriptable> {
    let path = PathBuf::from("r6")
        .join("config")
        .join(name)
        .with_extension("toml");

    std::fs::read_to_string(path)
        .ok()
        .and_then(|contents| contents.parse::<Value>().ok())
        .map(construct_value)
        .unwrap_or(Ref::null())
}

fn construct_value(val: Value) -> Ref<RED4ext::IScriptable> {
    match val {
        Value::String(str) => {
            call!("Toml.StringValue::New;String" (str) -> Ref<RED4ext::IScriptable>)
        }
        Value::Integer(i) => {
            call!("Toml.IntValue::New;Int64" (i) -> Ref<RED4ext::IScriptable>)
        }
        Value::Float(f) => {
            call!("Toml.FloatValue::New;Double" (f) -> Ref<RED4ext::IScriptable>)
        }
        Value::Boolean(b) => {
            call!("Toml.BoolValue::New;Bool" (b) -> Ref<RED4ext::IScriptable>)
        }
        Value::Datetime(dt) => {
            call!("Toml.StringValue::New;String" (dt.to_string()) -> Ref<RED4ext::IScriptable>)
        }
        Value::Array(arr) => {
            let inst = call!("Toml.ArrayValue::New;" () -> Ref<RED4ext::IScriptable>);
            for val in arr {
                let converted = construct_value(val);
                call!(inst.clone(), "Push" (converted) -> ());
            }
            inst
        }
        Value::Table(map) => {
            let inst = call!("Toml.TableValue::New;" () -> Ref<RED4ext::IScriptable>);
            for (key, val) in map {
                let converted = construct_value(val);
                call!(inst.clone(), "Push" (key, converted) -> ());
            }
            inst
        }
    }
}
