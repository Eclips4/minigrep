use crate::base;
use base::{Information, Search};


fn search_sensitive<'a>(query: &'a str, contents: &'a str)
    -> Search<'a> {
    let mut results = vec![];
    let mut counter: u32 = 1;
    for line in contents.split("\n") {
        if line.contains(&query) {
            results.push(Information{ string: line, line_number: counter });
        }
        counter += 1;
    }
    if !results.is_empty(){
        Some(results)
    } else {
        None
    }
}

fn search_insensitve<'a>(query: &'a str, contents: &'a str)
    -> Search<'a> {
    let mut results = vec![];
    let mut counter: u32 = 1;
    for line in contents.split("\n") {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(Information{ string: line, line_number: counter })
        }
        counter += 1;
    }
    if !results.is_empty() {
        Some(results)
    } else {
        None
    }
}


pub fn search<'a>(query: &'a str, contents: &'a str, is_sensitive: bool) -> Search<'a> {
    let results: Search<'a>;
    if is_sensitive {
        results = search_sensitive(&query, &contents);
    } else {
        results = search_insensitve(&query, &contents);
    }
    match results {
        Some(information) => Some(information),
        None => None,
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sensitive_empty(){
        let query = "rust";
        let contents = "\
First line
Rust is a good language.
Last line
";
        let results = search_sensitive(query, contents);
        assert!(results.is_none())
    }
    #[test]
    fn sensitive_result(){
        let query = "Last";
        let contents = "\
First line
Rust is a good language.
Last line
";
        let results = search_sensitive(query, contents).unwrap();
        let info = &results[0];
        assert_eq!(info.string, "Last line");
        assert_eq!(info.line_number, 3);
    }
    #[test]
    fn insensitive_empty(){
        let query = "second";
        let contents = "\
First line
Rust is a good language.
Last line
";
        let results = search_insensitve(query, contents);
        assert!(results.is_none());
    }
    #[test]
    fn insensitive_result(){
        let query = "rust";
        let contents = "\
First line
Rust is a good language.
Last line
";
        let results = search_insensitve(query, contents).unwrap();
        let info = &results[0];
        assert_eq!(info.string, "Rust is a good language.");
        assert_eq!(info.line_number, 2);
    }
}
