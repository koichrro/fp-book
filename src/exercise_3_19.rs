fn first_two(v: Vec<&str>) -> Vec<&str> {
    if v.len() < 2 {
        return v;
    }
    return v[0..2].to_vec().clone();
}

fn last_two(v: Vec<&str>) -> Vec<&str> {
    if v.len() < 2 {
        return v;
    }
    return v[v.len() - 2..].to_vec().clone();
}

fn moved_first_two_to_end(v: Vec<&str>) -> Vec<&str> {
    let mut ft = first_two(v.clone());
    let mut rest = v[2..].to_vec().clone();

    rest.append(&mut ft);
    return rest;
}

fn insert_before_last<'a>(v: Vec<&'a str>, new_element: &'a str) -> Vec<&'a str> {
    let mut nv = v.clone();
    nv.insert(nv.len()-1, new_element);
    return nv;
}

#[cfg(test)]
mod tests {
    use crate::exercise_3_19::{first_two, last_two, moved_first_two_to_end, insert_before_last};

    #[test]
    fn test_first_two() {
        assert_eq!(first_two(vec!["a", "b", "c"]), vec!["a", "b"]);
    }

    #[test]
    fn test_last_two() {
        assert_eq!(last_two(vec!["a", "b", "c"]), vec!["b", "c"]);
    }

    #[test]
    fn test_moved_first_two_to_end() {
        assert_eq!(
            moved_first_two_to_end(vec!["a", "b", "c"]),
            vec!["c", "a", "b"]
        );
    }

    #[test]
    fn test_insert_before_last() {
        assert_eq!(insert_before_last(vec!["a", "b"], "c"), vec!["a", "c", "b"]);
    }
}
