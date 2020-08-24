use heck::{CamelCase, KebabCase, MixedCase, SnakeCase, TitleCase};
use std::collections::HashMap;
use tera::{to_value, try_get_value, Result, Value};

pub fn camel_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("camel_case", "value", String, value);
    Ok(to_value(&s.to_camel_case()).unwrap())
}

pub fn kebab_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("kebab_case", "value", String, value);
    Ok(to_value(&s.to_kebab_case()).unwrap())
}

pub fn lower_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("lower_case", "value", String, value);
    Ok(to_value(&s.to_lowercase()).unwrap())
}

pub fn mixed_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("mixed_case", "value", String, value);
    Ok(to_value(&s.to_mixed_case()).unwrap())
}

pub fn snake_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("snake_case", "value", String, value);
    Ok(to_value(&s.to_snake_case()).unwrap())
}

pub fn title_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("title_case", "value", String, value);
    Ok(to_value(&s.to_title_case()).unwrap())
}

pub fn upper_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("upper_case", "value", String, value);
    Ok(to_value(&s.to_uppercase()).unwrap())
}
