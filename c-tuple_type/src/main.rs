// Tuples are way of grouping varaibles of different types with each position having a type
// Tuples have a fixed length, can't decrease or increase
// Parentheses () are used in grouping the variables

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    // Example
    let tup_a:(f64, i32, &str) = (2.3, 56, "Hello"); // Each position has a type

    // Items in Tuple are indexed and are accessed with a . instead of []
    let str_a: &str = tup_a.2; // assigns the value "Hello" to the varable

    println!("{}", str_a);

    {/* A tuple can also be Destructured */}
    let tup_b: (&str, i32, f64) = ("Hi", 34, 8.7);

    let (x, y, z) = tup_b;
    println!("The values in the tuple are: {}, {} and {} ", x, y, z);

    {/* If you want to destructure only a specific value, _ can be used */}
    let (_, _, my_num) = tup_b;

    println!("Destructured {} from a tuple", my_num);

    {/* Empty Tuples */}
    fn empty_tuple() {} // when a function doesn't return anything, it returns an empty tuple
}
