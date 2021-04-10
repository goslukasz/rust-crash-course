pub fn run() {
    greeting("Hello", "Łukasz");
    println!("add 1 + 2  = {}", add(1, 2));

    // Closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("Closure sum: {}", add_nums(3, 3));

    // Closure v2
    let n3: i32 = 10;
    let add_nums2 = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure2 sum: {}", add_nums2(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {} nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}