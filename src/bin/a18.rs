use std::io::Result;

// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

#[derive(Debug)]
enum LegalAge {
    Restricted,
    Unrestricted,
}

#[derive(Debug)]
struct Customer {
    age: i32,
    name: String,
}

fn is_legal() -> Result<(), String> {
    match legal {
        Ok(restrict) => print_restriction(restrict),
        _ => Err("age must be a number".to_owned()),
    };
}

fn print_restriction(restrict: Result<(), _>) {
    println!("restricted: {:?}", restrict);
}

fn main() {
    let victor = Customer {
        age: 23,
        name: "Victor".to_owned(),
    };
    let restrict = is_legal(victor);
    print_restriction(restrict);
}
