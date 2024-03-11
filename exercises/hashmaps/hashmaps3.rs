// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.


/**
 * 在代码中，为什么在访问 scored 引用的成员时不需要显式解引用，而在其他情况下需要呢？
 * 
 * 在 Rust 中，当有一个引用时，可以直接使用.操作符来访问结构体或枚举的字段或方法，而不需要显式解引用。
 * 这是因为 Rust 编译器能够自动解引用引用并访问相应的字段或方法。
 * 在代码片段中，scored 是一个可变引用，指向了 HashMap 中某个键对应的值，该值是一个 Team 结构体。
 * 因为 scored 是一个引用，而 Team 结构体的字段是直接的成员，
 * 所以可以直接通过 scored.goals_scored 和 scored.goals_conceded 访问这些成员，而不需要手动解引用。
 * 这种自动解引用使得代码更加清晰简洁，因为可以直接在引用上使用.操作符，而不需要手动解引用。
 */
use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.
        let mut scored = scores.entry(team_1_name).or_insert(Team { goals_scored: 0, goals_conceded: 0 });
        
        /* ⚠️WARNING!️⚠️️This is a wrong exp */
        // *scored.goals_scored += team_1_score;
        
        scored.goals_scored += team_1_score;
        scored.goals_conceded += team_2_score;

        scored = scores.entry(team_2_name).or_insert(Team { goals_scored: 0, goals_conceded: 0 });
        scored.goals_scored += team_2_score;
        scored.goals_conceded += team_1_score;
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
