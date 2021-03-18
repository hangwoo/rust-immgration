type LanguageResult<'a> = Result<(), &'a str>;

pub fn match_japanese_char(data: &char) -> bool {
    match *data {
        '\u{3041}'..='\u{3096}' => true,
        '\u{30A0}'..='\u{30FF}' => true,
        '\u{3400}'..='\u{4DB5}' => true,
        '\u{4E00}'..='\u{9FCB}' => true,
        '\u{F900}'..='\u{FA6A}' => true,
        '\u{2E80}'..='\u{2FD5}' => true,
        '\u{FF5F}'..='\u{FF9F}' => true,
        '\u{3000}'..='\u{303F}' => true,
        '\u{31F0}'..='\u{31FF}' => true,
        '\u{3220}'..='\u{3243}' => true,
        '\u{3280}'..='\u{337F}' => true,
        '\u{FF01}'..='\u{FF5E}' => true,
        _ => false,
    }
}

pub fn match_korean_char(data: &char) -> bool {
    match *data {
        '\u{01100}'..='\u{011FF}' => true,
        '\u{03001}'..='\u{03003}' => true,
        '\u{03008}'..='\u{03011}' => true,
        '\u{03013}'..='\u{0301F}' => true,
        '\u{0302E}'..='\u{03030}' => true,
        '\u{03037}' => true,
        '\u{030FB}' => true,
        '\u{03131}'..='\u{0318E}' => true,
        '\u{03200}'..='\u{0321E}' => true,
        '\u{03260}'..='\u{0327E}' => true,
        '\u{0A960}'..='\u{0A97C}' => true,
        '\u{0AC00}'..='\u{0D7A3}' => true,
        '\u{0D7B0}'..='\u{0D7C6}' => true,
        '\u{0D7CB}'..='\u{0D7FB}' => true,
        '\u{0FE45}'..='\u{0FE46}' => true,
        '\u{0FF61}'..='\u{0FF65}' => true,
        '\u{0FFA0}'..='\u{0FFBE}' => true,
        '\u{0FFC2}'..='\u{0FFC7}' => true,
        '\u{0FFCA}'..='\u{0FFCF}' => true,
        '\u{0FFD2}'..='\u{0FFD7}' => true,
        '\u{0FFDA}'..='\u{0FFDC}' => true,
        _ => false,
    }
}

pub fn match_english_char(data: &char) -> bool {
    match *data {
        'a'..='z' => true,
        'A'..='Z' => true,
        _ => false,
    }
}

pub fn get_matcher(language_code: &str) -> fn(char: &char) -> bool {
    match language_code {
        "ko" => match_korean_char,
        "en" => match_english_char,
        "ja" => match_japanese_char,
        _ => panic!("wrong language_code"),
    }
}

pub fn language_checker<'a>(language_code: &str, str: & 'a str) -> LanguageResult<'a> {
    for word in str.split(" ") {
        for character in word.chars() {
            // TODO 특수문자 체크
            if !get_matcher(language_code)(&character) {
                return Err(word);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{language_checker};

    #[test]
    fn korean_test() {
        assert_eq!(language_checker("ko", "한글"), Ok(()));
        assert_eq!(language_checker("ko","english"), Err("english"));
        assert_eq!(language_checker("ko","カタカナ"), Err("カタカナ"));
        assert_eq!(language_checker("ko","ひらがな"), Err("ひらがな"));
        assert_eq!(language_checker("ko","漢字"), Err("漢字"));
    }

    #[test]
    fn japanese_test() {
        assert_eq!(language_checker("ja","カタカナ"), Ok(()));
        assert_eq!(language_checker("ja","ひらがな"), Ok(()));
        assert_eq!(language_checker("ja","漢字"), Ok(()));
        assert_eq!(language_checker("ja","english"), Err("english"));
        assert_eq!(language_checker("ja","한글"), Err("한글"));
    }

    #[test]
    fn english_test() {
        assert_eq!(language_checker("en","english"), Ok(()));
        assert_eq!(language_checker("en","한글"), Err("한글"));
        assert_eq!(language_checker("en","カタカナ"), Err("カタカナ"));
        assert_eq!(language_checker("en","ひらがな"), Err("ひらがな"));
        assert_eq!(language_checker("en","漢字"), Err("漢字"));
    }

    #[test]
    #[should_panic]
    fn wrong_language_code() {
        language_checker("fr", "test").unwrap();
    }
}