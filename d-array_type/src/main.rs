// Arrays are a collection of values of the same type
// They have fixed length
// They are stored on the Stack

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    // Examples
    {/*Because the elements in an array are of the same type, an Array type is [type; number of elemnts]*/}
    let arr_a: [i32; 5] = [1, 2, 3, 4, 5]; // notice (;) is used to seperate the type from the number of elems
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let arr_b: [i32; 4] = [5; 4]; // => [5, 5, 5, 5]
    println!("{:?}", arr_b);

    // Indexes are accessed with []
    let first_month = months[0];
    let last_month = months[11];
    println!("First month of the Year is {} and last is {}.", first_month, last_month);

    // Part of an Array can be sliced by using .. to show range

    let letters = ['a','b','c','d','e','f','g','h','i','j'];

    let slice_a_to_e = &letters[0..5]; // 5 not inclusive
    println!("{:?} ", slice_a_to_e);

    let slice_f_to_j = &letters[5..];
    println!("{:?} ", slice_f_to_j);

}
