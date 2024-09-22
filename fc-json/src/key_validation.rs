use std::{cell::RefCell, collections::HashMap};

use clap::builder::Str;
use serde_json::Value;
use colored::*;
use crate::json_utils::traverse_json;

fn print_result(path:&str, value:&str, is_match:bool, note:&str){
    println!(
        "{:75}\t{:10?}\t{:4}\t{}", 
        path, 
        value,
        if is_match { "#YES".green() } else { "#NO".red().bold() },
        note.bold()
    );
}

pub fn validate(input: &Value, keys: Vec<String>) -> bool {
    let mut map = HashMap::new();
    // let mut cb_map: HashMap<String, String> = HashMap::new();
    for s in keys {
        let parts: Vec<&str> = s.split(':').collect();
        map.insert(parts[0].to_string(), parts[1].to_string());
    }
     let ref_map = RefCell::new(map);
    


    let callback = |path:&str, value_type:&str|{
        let mut x = ref_map.borrow_mut();
        match x.get(path) {
            Some(field_type) => {
                print_result(path, field_type, true, &String::new());
                x.remove(path);
            }
            None => {
                // print_result(path, "-", false, "Not Found");
            }
        }
        
        // ref_map.borrow_mut().insert("AAAAAAAAA".to_string(), "BBBBBBBB".to_string());
        // let search_path = path;
        // match map.get(search_path) {
        //     Some(field_type) => {
        //         map.remove(path);
        //         println!("{:30} | {}", search_path, if field_type == value_type {"OK".green()} else {"MISMATCH".red().bold()})
        //     },
        //     None => {},
        // }
    };
    traverse_json(input, "".to_string(), &callback);

    true
}

#[test]
fn test_check_keys() {
    let data = r#"
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
    let keys = vec![
        "key_str:String".to_string(),
        "key_obj.key_obj_key_obj.key:String".to_string(),
        "key_obj.key_obj_key_obj.key2:Number".to_string()
    ];
    let j: Value  = serde_json::from_str(data).expect("Couldn't read input");
    assert!(validate(&j,keys));

}
