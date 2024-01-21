#[derive(Debug, PartialEq)]
struct Book<'a> {
    title: &'a str,
    authors: Vec<&'a str>,
}

impl<'a> Book<'a> {
    fn new(title: &'a str, authors: Vec<&'a str>) -> Self {
        Book { title, authors }
    }
}

fn recommend_books(friend: &str) -> Vec<Book> {
    let scala = vec![
        Book::new("FP in Scala", vec!["Chiusano", "Bjarnason"]),
        Book::new("Get Programming with Scala", vec!["Sfregola"]),
    ];

    let fiction = vec![
        Book::new("Harry Potter", vec!["Rowling"]),
        Book::new("The Lord of the Rings", vec!["Tolkien"]),
    ];

    return match friend {
        "Alice" => scala,
        "Bob" => fiction,
        _ => vec![],
    };
}

#[cfg(test)]
mod tests {
    use super::{recommend_books, Book};

    #[test]
    fn test_recommend() {
        let friends = ["Alice", "Bob", "Charlie"];
        let recommendations: Vec<Book> = friends
            .iter()
            .flat_map(|friend| recommend_books(*friend))
            .collect();
        assert_eq!(
            recommendations,
            vec![
                Book::new("FP in Scala", vec!["Chiusano", "Bjarnason"]),
                Book::new("Get Programming with Scala", vec!["Sfregola"]),
                Book::new("Harry Potter", vec!["Rowling"]),
                Book::new("The Lord of the Rings", vec!["Tolkien"]),
            ]
        );

        let authors: Vec<&str> = recommendations
            .iter()
            .flat_map(|book| book.authors.clone())
            .collect();
        assert_eq!(
            authors,
            &["Chiusano", "Bjarnason", "Sfregola", "Rowling", "Tolkien"]
        );
    }
}
