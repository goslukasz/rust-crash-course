// Arrays - Fixed list where elements are the same data type

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    // Re-assign
    numbers[1] = 20;

    // All values
    println!("All: {:?}", numbers);
    
    // Get single val
    println!("Single val:  {}" , numbers[0]);

    // Array length
    println!("Array length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get array reference
    let reference_array: &[i32] = &numbers;
    println!("Array reference: {:?}", reference_array);

    // Get array slice
    let slice: &[i32] = &numbers[0..2];
    println!("Array slice 0..2: {:?}", slice);
}