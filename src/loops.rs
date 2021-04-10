pub fn run() {
    let mut count = 0;

    //infinite loop
    loop {
        count += 1;
        println!("Number:  {}", count);

        if count == 20 {
            break;
        }
    }
/*
    // While Loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 3 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count );
        }

        //Inc
        count += 1;
    }
*/

    // For range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 3 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x );
        }  
    }
}