
pub mod convert {

    use std::collections::HashMap;

    pub fn to_pig_latin(original: &String) -> String {
        let first_letter = original.chars().next();
        let mut map = HashMap::new();

        map.insert('A', 'a');
        map.insert('E', 'e');
        map.insert('I', 'i');
        map.insert('O', 'o');
        map.insert('U', 'u');
        map.insert('Y', 'y');

        for (upper, lower) in map {
            if first_letter == Some(upper) || first_letter == Some(lower) {
                let converted = format!("{}-fay", original);
                return converted;
            }
        }
        let converted = format!("{}-{}ay", &original[1..], first_letter.unwrap());
        return converted;
    }
}