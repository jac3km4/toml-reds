use std::collections::HashMap;
use std::path::{Path, PathBuf};

use red4ext_rs::prelude::*;
use static_init::dynamic;
use toml::value::Map;
use toml::Value;

define_trait_plugin! {
    name: "toml4reds",
    author: "jekky",
    plugin: TomlPlugin
}

struct TomlPlugin;

impl Plugin for TomlPlugin {
    fn version() -> Version {
        Version::new(0, 0, 4)
    }

    fn post_register() {
        register_function!("Toml.LoadConfig", |str: String| load_config(&str)
            .unwrap_or_else(Ref::null));
        register_function!("Toml.SaveConfig", save_config);
    }

    fn unload() {
        flush_configs()
    }
}

#[dynamic]
static mut DEFERRED_CONFIGS: HashMap<String, Value> = HashMap::new();

fn load_config(name: &str) -> Option<Ref<RED4ext::IScriptable>> {
    let contents = std::fs::read_to_string(get_config_path(name)?).ok()?;
    let value = contents.parse().ok()?;
    Some(construct_value(value))
}

fn save_config(name: String, config: Ref<RED4ext::IScriptable>) {
    let val = deconstruct_value(config);
    DEFERRED_CONFIGS.write().insert(name, val);
}

fn flush_configs() {
    for (name, conf) in &*DEFERRED_CONFIGS.read() {
        let contents = toml::to_string_pretty(conf).unwrap();
        std::fs::write(get_config_path(name).unwrap(), contents).unwrap();
    }
}

fn get_config_path(name: &str) -> Option<PathBuf> {
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
    Some(path)
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
                call!(inst.clone(), "AddEntry" (key, converted) -> ());
            }
            inst
        }
    }
}

fn deconstruct_value(scriptable: Ref<RED4ext::IScriptable>) -> Value {
    match red4ext_rs::rtti::get_type_name(scriptable.clone())
        .to_string_lossy()
        .as_ref()
    {
        "Toml.StringValue" => {
            let str = call!(scriptable, "Get" () -> String);
            Value::String(str)
        }
        "Toml.IntValue" => {
            let i = call!(scriptable, "Get" () -> i64);
            Value::Integer(i)
        }
        "Toml.FloatValue" => {
            let f = call!(scriptable, "Get" () -> f64);
            Value::Float(f)
        }
        "Toml.BoolValue" => {
            let bool = call!(scriptable, "Get" () -> bool);
            Value::Boolean(bool)
        }
        "Toml.ArrayValue" => {
            let values = call!(scriptable, "Get" () -> Vec<Ref<RED4ext::IScriptable>>);
            let mut buf = Vec::with_capacity(values.len());
            for val in values {
                buf.push(deconstruct_value(val));
            }
            Value::Array(buf)
        }
        "Toml.TableValue" => {
            let mut map = Map::new();
            let keys = call!(scriptable.clone(), "GetKeys" () -> Vec<String>);
            for key in keys {
                let entry = call!(scriptable.clone(), "GetEntry" (key.as_str()) -> Ref<RED4ext::IScriptable>);
                map.insert(key, deconstruct_value(entry));
            }
            Value::Table(map)
        }
        _ => unreachable!(),
    }
}
