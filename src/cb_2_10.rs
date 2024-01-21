fn tip_percentage(names: Vec<String>) -> i8 {
    let total = names.len();
    return if total == 0 {
        0
    } else if total < 6 {
        10
    } else {
        20
    };
}

fn addPerson(names: Vec<String>, new_name: String) -> Vec<String> {
    let mut added = names.clone();
    added.push(new_name);
    return added;
}

