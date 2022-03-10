#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Hannu"), 32);
    scores.insert(String::from("Nea"), 22);

    let teams = vec!["Blue".to_string(), "Red".to_string()];
    let initial_scores = vec![10, 64];

    let mut scores : HashMap <_,_> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    
    let score = scores.get("Blue");
    if let Some(v) = score {
        println!("Blue scored {}", v);
    }

    for (key, value) in &scores {
        println!("{} has a score of {}", key, value);
    }

    scores.insert(String::from("Orange"), 5);
    scores.insert(String::from("Orange"), 10); // Team orange now has a score of 10
    scores.entry(String::from("Cyan")).or_insert(50);
    scores.entry(String::from("Cyan")).or_insert(25);   // Cyan will still have score of 50
    // scores.entry returns an enum called Entry that either EXISTS or NOTEXISTS
    // enum Entry has a method called or_insert that handles everything properly

    let x = scores.entry(String::from("Redxyan")); // X is an entry
    println!("{:?}", x);
    x.or_insert(5); // 
    println!("{:?}", scores);

    let text = "hello world hello wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count :&mut i32 = &mut map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    let mut monsters = HashMap::new();
    monsters.insert("Horribly".to_string(), 500);
    monsters.insert("Jackathon".to_string(), 500);
    monsters.insert("Lobanator".to_string(), 500);

    if monsters.contains_key("horribly") {
        println!("Contains");
    }
    else {
        println!("Does not contain");
    }
}
