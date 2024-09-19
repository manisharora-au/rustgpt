use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collections_hashmap() {

        let person1 = "alice";
        let person2 = "bob";

        let mut persons: HashMap<&str, u32> = HashMap::new();
        persons.insert(person1, 55);
        persons.insert(person2, 51);

        let result = persons.get(person1);
        dbg!("{}", result.unwrap());
        assert_eq!(Some(&55), result);
    }

    #[test]
    fn test_collections_hashset() {
        let mut names = HashSet::new();
        names.insert("Alice");
        names.insert("Bob");

        if names.contains("Alice") {
            dbg!("names contain Alice");
        }

        let result = names.contains(&"Alice");
        assert_eq!(true, result);

        names.remove(&"Alice");
        let result = names.contains(&"Alice");
        assert_eq!(false, result);

        let result = names.contains(&"Charlie");
        assert_eq!(false, result);
    }
}

