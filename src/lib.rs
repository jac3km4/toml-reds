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

fn load_config(name: &str) -> Option<Ref<ffi::IScriptable>> {
    let contents = std::fs::read_to_string(get_config_path(name)?).ok()?;
    let value = contents.parse().ok()?;
    Some(construct_value(value))
}

fn save_config(name: String, config: Ref<ffi::IScriptable>) {
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

fn construct_value(val: Value) -> Ref<ffi::IScriptable> {
    match val {
        Value::String(str) => {
            call!("Toml.StringValue::New;String" (str) -> Ref<ffi::IScriptable>)
        }
        Value::Integer(i) => {
            call!("Toml.IntValue::New;Int64" (i) -> Ref<ffi::IScriptable>)
        }
        Value::Float(f) => {
            call!("Toml.FloatValue::New;Double" (f) -> Ref<ffi::IScriptable>)
        }
        Value::Boolean(b) => {
            call!("Toml.BoolValue::New;Bool" (b) -> Ref<ffi::IScriptable>)
        }
        Value::Datetime(dt) => {
            call!("Toml.StringValue::New;String" (dt.to_string()) -> Ref<ffi::IScriptable>)
        }
        Value::Array(arr) => {
            let inst = call!("Toml.ArrayValue::New;" () -> Ref<ffi::IScriptable>);
            for val in arr {
                let converted = construct_value(val);
                call!(inst, "Push" (converted) -> ());
            }
            inst
        }
        Value::Table(map) => {
            let inst = call!("Toml.TableValue::New;" () -> Ref<ffi::IScriptable>);
            for (key, val) in map {
                let converted = construct_value(val);
                call!(inst, "AddEntry" (key, converted) -> ());
            }
            inst
        }
    }
}

fn deconstruct_value(scriptable: Ref<ffi::IScriptable>) -> Value {
    match rtti::get_type_name(rtti::get_scriptable_type(scriptable)) {
        toml_value::STRING => {
            let str = call!(scriptable, "Get" () -> String);
            Value::String(str)
        }
        toml_value::INT => {
            let i = call!(scriptable, "Get" () -> i64);
            Value::Integer(i)
        }
        toml_value::FLOAT => {
            let f = call!(scriptable, "Get" () -> f64);
            Value::Float(f)
        }
        toml_value::BOOL => {
            let bool = call!(scriptable, "Get" () -> bool);
            Value::Boolean(bool)
        }
        toml_value::ARRAY => {
            let values = call!(scriptable, "Get" () -> Vec<Ref<ffi::IScriptable>>);
            let mut buf = Vec::with_capacity(values.len());
            for val in values {
                buf.push(deconstruct_value(val));
            }
            Value::Array(buf)
        }
        toml_value::TABLE => {
            let mut map = Map::new();
            let keys = call!(scriptable, "GetKeys" () -> Vec<String>);
            for key in keys {
                let entry = call!(scriptable, "GetEntry" (key.as_str()) -> Ref<ffi::IScriptable>);
                map.insert(key, deconstruct_value(entry));
            }
            Value::Table(map)
        }
        _ => unreachable!(),
    }
}

mod toml_value {
    use red4ext_rs::prelude::CName;

    pub const STRING: CName = CName::new("Toml.StringValue");
    pub const INT: CName = CName::new("Toml.IntValue");
    pub const FLOAT: CName = CName::new("Toml.FloatValue");
    pub const BOOL: CName = CName::new("Toml.BoolValue");
    pub const ARRAY: CName = CName::new("Toml.ArrayValue");
    pub const TABLE: CName = CName::new("Toml.TableValue");
}
