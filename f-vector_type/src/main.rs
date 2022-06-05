// Vectors are a collection of values of the same type
// The length is not fixed
// It is stored in the heap

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    // Examples
    let mut my_numbers: Vec<i32> = Vec::new(); // Creating an empty vector to hold values of type i32
    // another way of creating a vector is to use vec! macro
    let vector_b = vec![1, 2, 3]; // Rust automatically knows the vector will  be of type Vec<i32> because we have inserted i32s
    let vector_c = vec!["one", "two", "three"]; // Vec<&str>

    {/* Modifying a Vector */}
    my_numbers.push(10); // push() is used to add values to the Vector
    my_numbers.push(20);
    my_numbers.push(30);
    println!("Updated my_numbers: {:?} ", my_numbers);

    {/* Reading Elements of Vectors */}
    let second_value: i32 = my_numbers[1]; // Indexing
    println!("Second element in my_numbers is: {:?} ", second_value);

    // Using the get() method
    match my_numbers.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    {/* Vectors with compound values */}
    let vec1: Vec<(i32, i32)> = vec![(3, 4), (45, 90), (2, 45)]; // vector that contains tuples of i32s
    let mut vec2: Vec<Vec<String>> = Vec::new();

    vec2.push(vec!["Hello".to_string(), "Hola".to_string()]);
    vec2.push(vec!["Bello".to_string(), "Bola".to_string()]);

    println!("Value of vec2: {:?}", vec2);

    {/* Slicing a Vector */}
    let letters: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i' ,'j' ];
    let vec_a_to_e = &letters[0..5];
    println!("Value of vec_a_to_e: {:?}", vec_a_to_e);

    println!("===============");
    println!("REALLOCATION");
    println!("===============");
    /*
    when a Vector is declared it has a capacity
    as more elements are added to the Vector, it
    gets closer to it's capacity. Once the capacity is exceeded,
    the capacity is doubled and the elements are copied to a new space on the heap.
    This is called REALLOCATION
    */
    // .capacity() is used to check the current capacity of a Vector
    // with_capacity() is used to give a Vector a capacity at declaration, this is to avoid reallocation

    let mut num_vec = Vec::new();
    println!("{}", num_vec.capacity()); // 0 elements: prints 0
    num_vec.push('a'); // add one character
    println!("{}", num_vec.capacity()); // 1 element: prints 4. Vecs with 1 item always start with capacity 4
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // 4 elements: still prints 4.
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // prints 8. We have 5 elements, but it doubled 4 to 8 to make space
    
    println!("=======Giving a capacity of 8=======");

    let mut num_vector = Vec::with_capacity(8); // Give it capacity 8
    num_vector.push('a'); // add one character
    println!("{}", num_vector.capacity()); // prints 8
    num_vector.push('a'); // add one more
    println!("{}", num_vector.capacity()); // prints 8
    num_vector.push('a'); // add one more
    println!("{}", num_vector.capacity()); // prints 8.
    num_vector.push('a'); // add one more
    num_vector.push('a'); // add one more // Now we have 5 elements
    println!("{}", num_vector.capacity()); // Still 8

    {/* Convert Array to Vector */}
    let my_vec: Vec<u8> = [1, 2, 3].into();
}
