type LanguageResult<'a> = Result<(), &'a str>;

fn match_japanese_char(data: &char) -> bool {
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

fn match_korean_char(data: &char) -> bool {
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

fn match_english_char(data: &char) -> bool {
    match *data {
        'a'..='z' => true,
        'A'..='Z' => true,
        _ => false,
    }
}


pub fn korean(str: &str) -> LanguageResult {
    for word in str.split(" ") {
        for character in word.chars() {
            if !match_korean_char(&character) {
                return Err(word);
            }
        }
    }
    Ok(())
}

pub fn japanese(str: &str) -> LanguageResult {
    for word in str.split(" ") {
        for character in word.chars() {
            if !match_japanese_char(&character) {
                return Err(word);
            }
        }
    }
    Ok(())
}

pub fn english(str: &str) -> LanguageResult {
    for word in str.split(" ") {
        for character in word.chars() {
            if !match_english_char(&character) {
                return Err(word);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod language_checker_tests {
    use super::*;

    #[test]
    fn korean_test() {
        assert_eq!(korean("한글"), Ok(()));
        assert_eq!(korean("english"), Err("english"));
        assert_eq!(korean("カタカナ"), Err("カタカナ"));
        assert_eq!(korean("ひらがな"), Err("ひらがな"));
        assert_eq!(korean("漢字"), Err("漢字"));
    }

    #[test]
    fn japanese_test() {
        assert_eq!(japanese("カタカナ"), Ok(()));
        assert_eq!(japanese("ひらがな"), Ok(()));
        assert_eq!(japanese("漢字"), Ok(()));
        assert_eq!(japanese("english"), Err("english"));
        assert_eq!(japanese("한글"), Err("한글"));
    }

    #[test]
    fn english_test() {
        assert_eq!(english("english"), Ok(()));
        assert_eq!(english("한글"), Err("한글"));
        assert_eq!(english("カタカナ"), Err("カタカナ"));
        assert_eq!(english("ひらがな"), Err("ひらがな"));
        assert_eq!(english("漢字"), Err("漢字"));
    }
}