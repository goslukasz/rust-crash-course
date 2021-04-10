// Vectors are resizable arrays

use std::mem; 

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // Re-assign
    numbers[1] = 20;

    // Add value to vector
    numbers.push(6);

    // Pop off last value
    let last_value = numbers.pop();
    println!("Removed value: {:?}", last_value);

    // All values
    println!("All: {:?}", numbers);
    
    // Get single val
    println!("Single val:  {}" , numbers[0]);

    // Vector length
    println!("Vector length: {}", numbers.len());

    //Vector are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Vector reference
    let reference_vector: &[i32] = &numbers;
    println!("Vector reference: {:?}", reference_vector);

    // Get Vector slice
    let slice: &[i32] = &numbers[0..2];
    println!("Vector slice 0..2: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Value: {}", x);
    }

     // Loop on mut values
     for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Changed array: {:?}", numbers);
}