fn increment(x: i32) -> i32 {
    return x + 1;
}

fn get_first_character(s: &str) -> String {
    return if let Some(first) = s.get(0..1) {
        first.to_string()
    } else {
        String::from(" ")
    };
}

fn word_score(word: &str) -> u32 {
    return word.replace("a", "").len() as u32;
}

#[cfg(test)]
mod tests {
    use crate::cb_1_11::{get_first_character, word_score};

    use super::increment;

    #[test]
    fn test_increment() {
        assert_eq!(increment(5), 6);
    }

    #[test]
    fn test_word_score_1() {
        assert_eq!(word_score("Scala"), 3);
    }

    #[test]
    fn test_word_score_2() {
        assert_eq!(word_score("hoge"), 4)
    }

    #[test]
    fn test_word_score_3() {
        assert_eq!(word_score(""), 0)
    }

    #[test]
    fn test_get_first_character_1() {
        assert_eq!(get_first_character("Scala"), "S");
    }

    #[test]
    fn test_get_first_character_2() {
        assert_eq!(get_first_character(""), " ");
    }

    #[test]
    fn test_get_first_character_3() {
        assert_eq!(get_first_character("  Ha!  "), " ");
    }
}
