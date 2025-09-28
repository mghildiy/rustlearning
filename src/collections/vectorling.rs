pub fn play_with_vectors() {
    println!("Welcome to world of rust vectors");

    println!("Create a mutable empty vector");
    let int_vec: Vec<i32> = Vec::new();

    println!("Create a immutable initialized vector");
    let ini_vec = vec![1, 2, 3];

    println!("Create a mutable vector");
    let mut mut_vec = Vec::new();
    mut_vec.push(1);
    mut_vec.push(2);
    mut_vec.push(3);
    mut_vec.push(4);
    mut_vec.push(5);
    println!("Accessing elements in a vector");
    mut_vec[0] = 11;
    println!("Reference that is mutable but cant mutate the value it is pointing to");
    let mut first_element_ref = &mut_vec[0];
    println!("Mutable reference before mutating it: {}", first_element_ref);
    first_element_ref = &6;
    println!("Mutable reference after mutating it: {}", first_element_ref);
    println!("First element in vector: {}", mut_vec[0]);
    println!("Third element in  vector: {}", mut_vec.get(2).unwrap());

    println!("Iterate vector");
    for i in &mut_vec {
        println!("{}", i);
    }
    for i in &mut mut_vec {
        *i += 50;
        println!("{}", i);
    }
}