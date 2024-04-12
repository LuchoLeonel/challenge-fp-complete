use std::fs;
use std::env;
use std::collections::HashSet;
use regex::Regex;


fn main() {
    fn parse_names(contents: &str) -> Vec<&str> {
        let regex = Regex::new(r#""[^"]*"|\S+"#).unwrap();
        regex.find_iter(contents)
             .map(|mat| mat.as_str().trim_matches('"'))
             .collect()
    }
    fn filter_names_only(data: Vec<&str>) -> Vec<&str> {
        data.into_iter()
            .filter(|&item| !item.chars().any(|c| c.is_digit(10)))
            .collect()
    }

    fn filter_unique_names(data: Vec<&str>) -> Vec<&str> {
        let mut seen = HashSet::new();
        data.into_iter()
            .filter(|&item| !item.chars().any(|c| c.is_digit(10)) && seen.insert(item))
            .collect()
    }

    #[derive(Debug, Clone)]
    struct PersonAccount {
        pub owner: String,
        pub amount: i32,
    }

    let current_dir = env::current_dir().unwrap();
    let path = format!("{}/src/public/transcriptions.txt", current_dir.display());
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    
    let parts: Vec<&str> = parse_names(&contents);

    let transactions: Vec<Vec<&str>> = parts.chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect();

    let persons = filter_unique_names(filter_names_only(parts));

    let mut person_accounts: Vec<PersonAccount> = persons.iter().map(|p| PersonAccount {
        owner: p.to_string(),
        amount: 0,
    }).collect::<Vec<PersonAccount>>();

    for transaction in transactions {
        let amount = transaction[0].parse::<i32>().unwrap();

        let transfer_index = person_accounts.iter().position(|acc| acc.owner == transaction[1]).unwrap();
        let receive_index = person_accounts.iter().position(|acc| acc.owner == transaction[2]).unwrap();
        
        person_accounts[transfer_index].amount -= amount;
        person_accounts[receive_index].amount += amount;
    }

    dbg!(&person_accounts);
    
}
