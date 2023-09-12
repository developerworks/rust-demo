fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let some_yaml = r#"
    [5,6]: true
    "#;

        let try_yaml = serde_yaml::from_str::<serde_yaml::Value>(some_yaml);
        let try_json = serde_yaml::from_str::<serde_json::Value>(some_yaml);

        assert!(try_yaml.is_ok());
        assert!(try_json.is_err());
    }
}
