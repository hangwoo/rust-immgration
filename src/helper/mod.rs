use std::fs::read_to_string;
use serde_json::{Value};
pub mod language_checker;

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

pub fn read_ignore_word_by_json(json_path: &str) -> Vec<String> {
    let result = serde_json::from_str::<Value>(&read_to_string(json_path).unwrap()).unwrap();
    if !result.is_array() {
        panic!("json must be array");
    }

    let mut ignore_words: Vec<String> = vec![];

    let arr = result.as_array().unwrap();

    for i in arr {
        if !i.is_string() {
            panic!("wrong word {}", i.as_str().unwrap());
        }
        ignore_words.push(i.as_str().unwrap().to_string());
    }

    ignore_words
}

#[cfg(test)]
mod tests {
    use crate::helper::{iterate_object, read_ignore_word_by_json};
    use serde_json::{Value};

    #[test]
    #[should_panic]
    fn iterate_object_test() {
        let v = Value::String("string".to_string());
        iterate_object(&v);
    }

    #[test]
    #[should_panic]
    fn read_ignore_word_by_json_wrong_path() {
        read_ignore_word_by_json("panic.json");
    }

    #[test]
    fn read_ignore_word_by_json_correct_json() {
        assert_eq!(read_ignore_word_by_json("tests/ignore.json"), vec!["word", "word2"]);
    }

    #[test]
    #[should_panic]
    fn read_ignore_word_by_json_wrong_json() {
        read_ignore_word_by_json("tests/wrong_ignore.json");
    }
}
