use serde_json::{Value};
mod language_checker;

pub fn iterate_object(value: &Value) {
    let json = value.as_object().unwrap();
    for (str, v) in json.iter() {
        if v.is_object() {
            iterate_object(v);
        } else {
            println!("key:{}, value:{}", str, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::iterate_object;
    use serde_json::{Value};

    #[test]
    #[should_panic]
    fn iterate_object_test() {
        let v = Value::String("string".to_string());
        iterate_object(&v);
    }

    #[test]
    #[should_panic]
    fn language_checker_test() {
    }
}
