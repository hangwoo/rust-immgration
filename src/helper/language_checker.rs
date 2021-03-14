pub fn korean() {

}

pub fn japanese() {

}

pub fn english() {

}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn korean_test() {
        assert_eq!(korean("한글"), true);
        assert_eq!(korean("english"), false);
        assert_eq!(korean("カタカナ"), false);
        assert_eq!(korean("ひらがな"), false);
        assert_eq!(korean("漢字"), false);
    }

    #[test]
    fn japanese_test() {
        assert_eq!(japanese("カタカナ"), true);
        assert_eq!(japanese("ひらがな"), true);
        assert_eq!(japanese("漢字"), false);
        assert_eq!(japanese("english"), false);
        assert_eq!(japanese("한글"), false);
    }

    #[test]
    fn english_test() {
        assert_eq!(english("english"), true);
        assert_eq!(english("한글"), false);
        assert_eq!(english("カタカナ"), false);
        assert_eq!(english("ひらがな"), false);
        assert_eq!(english("漢字"), false);
    }
}