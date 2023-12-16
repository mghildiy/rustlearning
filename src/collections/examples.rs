pub fn vectorDemo() {
    // immutable empty vector, type to be annotated
    let emptyVec: Vec<i32> = Vec::new();
    let nonemptyVec = vec![10,20,30];
    // mutable empty vector, no need to annotate type as push function call tells compiler the type
    let mut anotherEmptyVec = Vec::new();
    anotherEmptyVec.push(1);
    anotherEmptyVec.push(111);
    anotherEmptyVec.push(111);
    anotherEmptyVec.push(1111);
    let mut second = anotherEmptyVec[1];
    second = 1000;
    println!("second element is {second}");
    let x = anotherEmptyVec[1];
    println!("x is {x}");
    anotherEmptyVec[0] = 77;
    let y = anotherEmptyVec[0];
    println!("y is {y}");
    let mut z = &anotherEmptyVec[0];
    //anotherEmptyVec.push(101);
    println!("z is {z}");

    let mayBe = anotherEmptyVec.get(2);
    match mayBe {
        Some(val) => println!("element is {val}"),
        _ => println!("None found"),
    }

    let mut nums = vec![100,200,300];
    for elem in &mut nums {
        *elem += 10;
        println!("value is: {elem}")
    }
    let first = nums[0];
    println!("{first}");
}