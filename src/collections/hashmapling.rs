use std::collections::HashMap;

pub fn play_with_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("a"), 100);
    scores.insert(String::from("b"), 50);
    let may_be_score = scores.get("a");
    let score = scores.get("b").copied();

    let mut capitals_nations = HashMap::new();
    capitals_nations.insert(String::from("delhi"), "india".to_string());
    let may_be_country = capitals_nations.get("delhi");
    let country = capitals_nations.get("delhi").unwrap_or(&"none".to_string());

    let moscow = "moscow".to_string();
    let russia = "russia".to_string();
    capitals_nations.insert(moscow.clone(), russia.clone());

    for (capital, country) in &capitals_nations {
        println!("{}: {}", capital, country);
    }

    let mut colors_scores = HashMap::new();
    colors_scores.insert(String::from("Blue"), 10);
    colors_scores.entry(String::from("Yellow")).or_insert(50);
    colors_scores.entry(String::from("Blue")).or_insert(50);

    println!("{colors_scores:?}");

}