// Struct is a way of creating your own custom data type
// structs hold related values of different types
// name struct should be written in CamelCase

#[allow(dead_code)]
// There are 3 types of structs:
 // 1) Unit Struct
 struct WaterFactory; // don't have fields, useful when implementing traits
 // 2) Tuple Struct
 struct Colour(u8, u8, u8); // here types are defined but no field names
 // 3) Named Struct
 struct MyProfile {
     name: String, // fields are defined with types
     fav_color: Colour
 }

 #[allow(unused_variables)]
fn main() {
    let black = Colour(0, 0, 0);

    let mut me = MyProfile { // creating an instance and assign values to fields
        name: String::from("Gift"),
        fav_color: black,
    };

    let my_name = me.name; // retrieve specific value
    let new_colour = Colour(234, 23, 255);

    me.fav_color = new_colour; // change value

    let person1 = user(
        String::from("Gift"),
 String::from("Nigerian"),
      true);

    let person2 = Person {
        name: String::from("Grace"),
        ..person1 
        /*
        since person1 and person2 share the same values for some fields(nationality & active),
        the struct update syntax `..` can be used to spread the rest of the values
        of the given instance 
         */
    };

    println!("person2 details are: {:?}", person2); // returns 'name' as "Grace" and returns the other values from person1
}


#[allow(dead_code)]
#[derive(Debug)]
struct Person { // new struct
    name: String,
    nationality: String,
    active: bool,
}

fn user (name: String, nationality: String, active: bool, ) -> Person {

    let person = Person {
        name, // since the parameter and field names are the same,
        nationality, // field init shorthand syntax is used and,
        active, // there's no repitition
    };
    person
}