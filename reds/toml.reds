module Toml

public native func LoadConfig(name: String) -> ref<ConfigValue>;

public abstract class ConfigValue {
    public func Fold(fold: ref<ConfigFold>) -> Variant;
    final public func ToVariant() -> Variant = this.Fold(new ToVariantFold());
    
    final public func AsString() -> ref<StringValue> = this as StringValue;
    final public func AsInt() -> ref<IntValue> = this as IntValue;
    final public func AsFloat() -> ref<FloatValue> = this as FloatValue;
    final public func AsBool() -> ref<BoolValue> = this as BoolValue;
    final public func AsArray() -> ref<ArrayValue> = this as ArrayValue;
    final public func AsTable() -> ref<TableValue> = this as TableValue;
}

public class StringValue extends ConfigValue {
    let value: String;

    public func Fold(fold: ref<ConfigFold>) -> Variant = fold.OnString(this.value);
    public func Get() -> String = this.value;

    static func New(value: String) -> ref<StringValue> {
        let self = new StringValue();
        self.value = value;
        return self;
    }
}

public class IntValue extends ConfigValue {
    let value: Int64;

    public func Fold(fold: ref<ConfigFold>) -> Variant = fold.OnInt(this.value);
    public func Get() -> Int64 = this.value;

    static func New(value: Int64) -> ref<IntValue> {
        let self = new IntValue();
        self.value = value;
        return self;
    }
}

public class FloatValue extends ConfigValue {
    let value: Double;

    public func Fold(fold: ref<ConfigFold>) -> Variant = fold.OnFloat(this.value);
    public func Get() -> Double = this.value;

    static func New(value: Double) -> ref<FloatValue> {
        let self = new FloatValue();
        self.value = value;
        return self;
    }
}

public class BoolValue extends ConfigValue {
    let value: Bool;

    public func Fold(fold: ref<ConfigFold>) -> Variant = fold.OnBool(this.value);
    public func Get() -> Bool = this.value;

    static func New(value: Bool) -> ref<BoolValue> {
        let self = new BoolValue();
        self.value = value;
        return self;
    }
}

public class ArrayValue extends ConfigValue {
    let values: array<ref<ConfigValue>>;

    public func Fold(fold: ref<ConfigFold>) -> Variant  {
        let converted: array<Variant>;
        for value in this.values {
            ArrayPush(converted, value.Fold(fold));
        }
        return converted;
    }

    public func Get() -> array<ref<ConfigValue>> = this.values;

    func Push(value: ref<ConfigValue>) {
        ArrayPush(this.values, value);
    }

    static func New() -> ref<ArrayValue> = new ArrayValue();
}

public class TableValue extends ConfigValue {
    let table: ref<inkHashMap>;

    public func Fold(fold: ref<ConfigFold>) -> Variant  {
        let values: array<wref<IScriptable>>;
        this.table.GetValues(values);

        let converted: array<Variant>;
        for item in values {
            ArrayPush(converted, (item as ConfigValue).Fold(fold));
        }
        return converted;
    }

    public func Get() -> ref<inkHashMap> = this.table;

    public func GetEntry(key: String) -> ref<ConfigValue> {
        return this.table.Get(TDBID.ToNumber(TDBID.Create(key))) as ConfigValue;
    }

    func Push(key: String, value: ref<ConfigValue>) {
        this.table.Insert(TDBID.ToNumber(TDBID.Create(key)), value);
    }

    static func New() -> ref<TableValue> {
        let self = new TableValue();
        self.table = new inkHashMap();
        return self;
    }
}

public abstract class ConfigFold {
    public func OnString(str: String) -> Variant;
    public func OnInt(int: Int64) -> Variant;
    public func OnFloat(float: Double) -> Variant;
    public func OnBool(bool: Bool) -> Variant;
}

public class ToVariantFold extends ConfigFold {
    public func OnString(str: String) -> Variant = ToVariant(str);
    public func OnInt(int: Int64) -> Variant = ToVariant(int);
    public func OnFloat(float: Double) -> Variant = ToVariant(float);
    public func OnBool(bool: Bool) -> Variant = ToVariant(bool);
}
