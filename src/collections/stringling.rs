pub fn play_with_string() {
    let immutable_string = "Hello, immutable world!".to_string();
    println!("{}", immutable_string);
    let mut mutable_string = String::from("Hello, mutable world");
    mutable_string.push('!');
    mutable_string.push_str("How are you mutating?");
    println!("{mutable_string}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let tic = "tic".to_string();
    let tac = "tac".to_string();
    let toe = "toe".to_string();
    let tic_tac_toe = format!("{tic}-{tac}-{toe}");
    println!("{tic_tac_toe}");

    let russ_str = "Зд".to_string();
    for c in russ_str.chars() {
        println!("{c}");
    }
    for b in russ_str.bytes() {
        println!("{b}");
    }

}