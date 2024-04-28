// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team 
// conceded. One approach to build the scores table is to use a Hashmap. 
// The solution is partially written to use a Hashmap, 
// complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

//! Ce programme construit une table des scores à partir des résultats de matchs. Il utilise une HashMap pour associer le nom d'une équipe à sa structure Team, contenant le nombre de buts marqués et encaissés.

use std::collections::HashMap;

struct Team { // Défini la structure Team
    goals_scored: u8,
    goals_conceded: u8,
}

/// Construit une table des scores à partir des résultats de matchs.
/// # Retour
///
/// Une HashMap associant le nom d'une équipe à sa structure Team.
fn build_scores_table(results: String) -> HashMap<String, Team> {
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect(); // Sépare la chaîne en fonction des virgules
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap(); // Convertit le nombre de buts marqués en u8
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap(); // Convertit le nombre de buts encaissés en u8

        let team_1 = scores.entry(team_1_name.clone()).or_insert(Team { goals_scored: 0, goals_conceded: 0 }); // Ajoute l'équipe 1 à la table des scores
        team_1.goals_scored += team_1_score; // Ajoute le nombre de buts marqués
        team_1.goals_conceded += team_2_score;

        let team_2 = scores.entry(team_2_name).or_insert(Team { goals_scored: 0, goals_conceded: 0 }); // Ajoute l'équipe 2 à la table des scores
        team_2.goals_scored += team_2_score; // Ajoute le nombre de buts encaissés
        team_2.goals_conceded += team_1_score;
    }

    scores // Retourne la table des scores
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

