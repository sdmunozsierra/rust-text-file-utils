#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_load_json_file() {
        let json_data = load_json_file("test.json").expect("Failed to load JSON file");
        assert!(json_data.is_object());
    }

    #[test]
    fn test_create_custom_object() {
        let json_data = json!({
            "id": 1,
            "name": "Test Object",
            "details": {
                "description": "This is a test",
                "nested_info": {
                    "info": "Some nested info"
                }
            }
        });

        let custom_object: CustomObject = create_custom_object(&json_data);
        assert_eq!(custom_object.id, 1);
        assert_eq!(custom_object.name, "Test Object");
    }

    #[test]
    fn test_get_nested_value() {
        let json_data = json!({
            "level1": {
                "level2": {
                    "level3": "value"
                }
            }
        });

        let keys = ["level1", "level2", "level3"];
        let value = get_nested_value(&json_data, &keys).expect("Failed to get nested value");
        assert_eq!(value, "value");
    }
}