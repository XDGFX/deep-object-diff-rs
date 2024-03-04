use serde_json::{Map, Result, Value};
use std::io::{self, BufRead};

fn convert_array_to_object(value: &Value) -> Value {
    match value {
        Value::Array(arr) => {
            let mut map = Map::new();
            for (index, val) in arr.iter().enumerate() {
                map.insert(index.to_string(), convert_array_to_object(val));
            }
            Value::Object(map)
        }
        Value::Object(obj) => {
            let mut new_obj = Map::new();
            for (key, val) in obj.iter() {
                new_obj.insert(key.clone(), convert_array_to_object(val));
            }
            Value::Object(new_obj)
        }
        _ => value.clone(),
    }
}

fn diff(lhs: &Value, rhs: &Value) -> Value {
    if lhs == rhs {
        return Value::Object(Map::new()); // equal return no diff
    }

    if (!lhs.is_object() && !lhs.is_array()) || (!rhs.is_object() && !rhs.is_array()) {
        return rhs.clone(); // return updated rhs
    }

    let lhs_temp = convert_array_to_object(&lhs);
    let lhs_obj = lhs_temp.as_object().unwrap();

    let rhs_temp = convert_array_to_object(&rhs);
    let rhs_obj = rhs_temp.as_object().unwrap();

    let mut deleted_values = Map::new();
    for key in lhs_obj.keys() {
        if !rhs_obj.contains_key(key) {
            deleted_values.insert(key.clone(), Value::Null);
        }
    }

    let mut result = deleted_values;
    for (key, rhs_value) in rhs_obj.iter() {
        if !lhs_obj.contains_key(key) {
            result.insert(key.clone(), rhs_value.clone()); // return added r key
            continue;
        }

        let difference = diff(&lhs_obj[key], rhs_value);

        // If the difference is empty, and the lhs is an empty object or the
        // rhs is not an empty object
        if difference.is_object() && difference.as_object().unwrap().is_empty() {
            if (lhs_obj[key].is_object() && lhs_obj[key].as_object().unwrap().is_empty())
                || (!rhs_value.is_object()
                    || rhs_value.is_object() && !rhs_value.as_object().unwrap().is_empty())
            {
                continue; // return no diff
            }
        }

        // Add the difference to the result
        result.insert(key.clone(), difference);
    }

    Value::Object(result)
}
fn main() -> Result<()> {
    let stdin = io::stdin(); // We get `Stdin` here.

    loop {
        let mut buffer1 = String::new();
        let mut buffer2 = String::new();

        stdin
            .lock()
            .read_line(&mut buffer1)
            .expect("Failed to read line");
        stdin
            .lock()
            .read_line(&mut buffer2)
            .expect("Failed to read line");

        let lhs: Value = serde_json::from_str(&buffer1)?;
        let rhs: Value = serde_json::from_str(&buffer2)?;

        println!("{}", diff(&lhs, &rhs));
    }
}
