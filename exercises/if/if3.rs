// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.


/**
 *  let 与 if 类型搭配赋值时 类型需相同
 */
pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        1i32
    } else if animal == "gopher" {
        2i32
    } else if animal == "snake" {
        3i32
    } else {
        4i32
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
