pub fn run() {
    // Primitive array
    let array1 = [1,2,3];
    let array2 = array1;

    println!("Arrays: {:?}", (array1, array2));

    // With non-primitives, if you assign another variable to piece of data the first variable no longer hold that value.
    // You will need to use reference & to point to the resource

    // Vector
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Vectors: {:?}", (&vec1, vec2));
}