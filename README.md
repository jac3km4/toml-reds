## toml-reds

### usage
- `r6/config/test.toml`
    ```toml
    [package]
    name = "toml"
    version = "0.4.2"
    ```

- `r6/scripts/testing.reds`
    ```swift
    import Toml.*

    func TestingToml() {
        let config = LoadConfig("test").AsTable();
        let package = config.GetEntry("package").AsTable();
        let name = package.GetEntry("name").AsString().Get();

        LogChannel(n"DEBUG", name);
    }
    ```
