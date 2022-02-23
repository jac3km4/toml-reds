## toml4reds

### usage
- `r6/config/test.toml`
    ```toml
    [package]
    name = "toml"
    ```

- `r6/scripts/testing.reds`
    ```swift
    import Toml.*

    func TomlSave() {
        let file = ConfigFile.Load("test");
        let config = file.Config().AsTable();
        let package = config.GetEntry("package").AsTable();
        let name = package.GetEntry("name").AsString();
        LogChannel(n"DEBUG", name.Get());

        name.Set("hello");
        file.Save();
    }
    ```

### notes
- saving config files is deferred until the game closes to avoid unnecessary IO, so you can't rely on a `Load` to observe changes made by `Save` in a single game run
  - if you want to keep track of your changes you should keep a reference to the ConfigFile or ConfigValue after loading
  - you can call Save as often as you like because it doesn't actually write to a file, it only stores the config to be written later
