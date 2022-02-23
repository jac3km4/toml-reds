## toml-reds

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
