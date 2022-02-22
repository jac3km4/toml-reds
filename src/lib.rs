use std::path::Path;

use red4ext_rs::prelude::*;
use toml::Value;

define_plugin! {
    name: "toml-reds",
    author: "jekky",
    version: 1:0:0,
    on_register: {
        register_function!("Toml.LoadConfig", load_config);
    }
}

fn load_config(name: String) -> Ref<RED4ext::IScriptable> {
    load_config_value(&name).unwrap_or(Ref::null())
}

fn load_config_value(name: &str) -> Option<Ref<RED4ext::IScriptable>> {
    let exe = std::env::current_exe().ok()?;
    let path: &Path = name.as_ref();
    let path = exe
        .parent()?
        .parent()?
        .parent()?
        .join("r6")
        .join("config")
        .join(path.file_name()?)
        .with_extension("toml");
    let contents = std::fs::read_to_string(path).ok()?;
    let value = contents.parse().ok()?;
    Some(construct_value(value))
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
