// There are two main types of strings
// 1.) String type
// 2.) &str type // this is pronou&nced as 'string slice' type

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    // Examples
    // 1.) String type
    let string_a: String = String::from("Rust is interesting!");

    // 2.) &str type
    let str_a: &str = "I love programming";

    // If you declare a string without specifying a type, by default is is a &str
    let sentence = "Hello World!"; // &str by default

    // They are both UTF-8
    let string_b: String = String::from("안녕하세요");
    let str_b: &str = "xin chào";

    {/* TRANSLATIONS */}

    // From &str to String
    let string_c: String = "Hola!".to_string();
    let string_d: String = str_b.to_string();
    let string_d: String = String::from(str_b);

    // From String to &str
    let str_c: &str = &string_a; // this reference the String

    {/* CONCATENATIONS */}

    let combo_string : String = ["Hello", "World"].concat(); // results in a String
    
    let combo_format_string = format!( "{}, {}, {}", str_a, sentence, string_b); // results in a String
    println!("{}", combo_format_string);

    // let combo_string_str = String + &str
    let combo_string_str: String = string_a + str_a; // the String type has to be first

    {/*All variables are immutable except they have the `mut` keyword */}

    let mut new_string = String::new(); // a String with no value
    new_string.push_str("Hola");

}

/*
Differences
─────────────────────────────────────────────────────────────────────────────────
│                 String                │             &str                      │ 
┌───────────────────────────────────────│───────────────────────────────────────┐
│- Stored in the heap                   │-Stored in the stack(occasionally heap)│
│- Mutable                              │- Immutable                            │
│- Owned Type                           │- Not owned                            │
│- Sized (it is always 24 bytes)        │- Dynamically sized                    │
└───────────────────────────────────────┴───────────────────────────────────────┘
*/