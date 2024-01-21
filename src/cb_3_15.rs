fn abbreviate(full_name: &str) -> String {
    if full_name.is_empty() {
        return "".to_string();
    }
    let v: Vec<&str> = full_name.trim().splitn(2, " ").collect();
    if v.len() != 2 {
        return full_name.to_string()
    }
    let first_name = v[0];
    let first_character = first_name.get(0..1).unwrap();
    return String::from(first_character) + ". " + v[1];
}

#[cfg(test)]
mod tests {
    use crate::cb_3_15::abbreviate;

    #[test]
    fn test_abbreviate_1() {
        assert_eq!(abbreviate("Alonzo Church"), "A. Church");
    }

    #[test]
    fn test_abbreviate_2() {
        assert_eq!(abbreviate("Church"), "Church");
    }

    #[test]
    fn test_abbreviate_3() {
        assert_eq!(abbreviate(" Church "), " Church ");
    }

    #[test]
    fn test_abbreviate_4() {
        assert_eq!(abbreviate("A. Church"), "A. Church");
    }

    #[test]
    fn test_abbreviate_5() {
        assert_eq!(abbreviate("A Church"), "A. Church");
    }
}