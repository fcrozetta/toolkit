use serde_json::Value;

pub fn traverse_json<F>(json: &Value, key_path: String, callback: &F)
where
    F: Fn(&str, &str),
{
    match json {
        Value::Object(map) => {
            for (key, value) in map {
                let new_key_path = if key_path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", key_path, key)
                };
                traverse_json(value, new_key_path, callback);
            }
        }
        Value::Array(array) => {
            if let Some(first_value) = array.get(0) {
                let element_type = determine_type(first_value);
                let array_type = format!("Array<{}>", element_type);
                
                // Recurse if the first element is an array or object to determine the full structure
                if matches!(first_value, Value::Array(_) | Value::Object(_)) {
                    traverse_json(first_value, format!("{}[]", key_path), callback);
                }else{
                    callback(&key_path, &array_type);
                }
            } else {
                // Empty array, call the callback with Array<Unknown>
                callback(&key_path, "Array<Unknown>");
            }
        }
        _ => {
            let value_type = determine_type(json);
            callback(&key_path, &value_type);
        }
    }
}

fn determine_type(json: &Value) -> String {
    match json {
        Value::String(_) => "String".to_string(),
        Value::Number(_) => "Number".to_string(),
        Value::Bool(_) => "Bool".to_string(),
        Value::Array(array) => {
            if let Some(first_value) = array.get(0) {
                let inner_type = determine_type(first_value);
                format!("Array<{}>", inner_type)
            } else {
                "Array<Unknown>".to_string()
            }
        }
        Value::Object(_) => "Object".to_string(),
        Value::Null => "Null".to_string(),
    }
}

#[test]
fn test() {
    let json_data = r#"
    {
        "key_str": "string",
        "key_bool": true,
        "key_number": 1,
        "key_arr_str": ["a","b","c"],
        "key_arr_int": [1,2,3],
        "key_arr_float": [1.0,2.0,3.0],
        "key_arr_arr": [[1,2,3],[4,5,6]],
        "key_arr_obj": [
            {
                "key_arr_obj_str": "string",
                "key_arr_obj_int": 1,
                "key_arr_obj_float": 2.0,
                "key_arr_obj_arr_int": [1,2,3],
                "key_arr_obj_arr_obj": [{"key_arr_obj_arr_obj_str":"a"}]
            }
        ],
        "key_obj": {
            "key_obj_key_str": "string",
            "key_obj_key_number": 1,
            "key_obj_key_arr_str": ["a","b","c"],
            "key_obj_key_arr_int": [1,2,3],
            "key_obj_key_arr_float": [1.0,2.0,3.0],
            "key_obj_key_arr_arr": [[1,2,3],[4,5,6]],
            "key_obj_key_obj": {
                "key":"value"
            }
        }
    }   
    "#;

    let parsed_json: Value = serde_json::from_str(json_data).unwrap();

    let callback = |key: &str, value_type: &str| {
        println!("{}: {}", key, value_type);
    };

    traverse_json(&parsed_json, "".to_string(), &callback);
}
