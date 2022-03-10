// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Bill {
    title: String,
    dollars: i32,
    cents: i32,
}

impl Bill {
    fn show(&self) {
        println!("Bill: {:?}", &self.title);
        if &self.cents < &10 {
            println!("Total: {:?}.0{:?}", &self.dollars, &self.cents)
        } else {
            println!("Total: {:?}.{:?}", &self.dollars, &self.cents)
        };
        println!("");
    }

    fn new(title: &str, dollars: i32, cents: i32) {
        let new_bill = Self {
            title: title.to_owned(),
            dollars,
            cents
        };
        let mut bills = HashMap::new();
        bills.insert(title, new_bill);
    }
}

#[derive(Debug)]
enum MainMenu {
    New,
    View,
    Edit,
    Delete,
    Quit
}

fn menu_new() {
    println!("Creating new bill...");
}

fn menu_view() {
    println!("Creating new bill...");
}
fn menu() -> io::Result<String> {
    let menu_string = "
    Billing App
Welcome to BillingApp 
commands:
    new      - Adds a new bill
    view     - View existing bills
    edit N/A - Edit an existing bill
    del  N/A - Delete an existing bill
    q        - Quit application
    ";
    println!("{}", menu_string);
    print!(":>");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.to_owned())
}

fn main() {
    let bill_one = Bill {
        title: "Electricity".to_owned(),
        dollars: 130,
        cents: 20
    };

    let bill_two = Bill {
        title: "Rent".to_owned(),
        dollars: 700,
        cents: 00
    };
    let main_menu = menu();


}
