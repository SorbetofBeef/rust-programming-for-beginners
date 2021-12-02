// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum TicketType {
    BackStage(String, i32),
    VIP(String, i32),
    Standard(i32),
}

fn main() {
    let tickets = vec![
        TicketType::BackStage("Gene".to_owned(), 125),
        TicketType::VIP("John".to_owned(), 75),
        TicketType::Standard(50),
    ];

    for ticket in tickets {
        match ticket {
            TicketType::VIP(name, price) => println!("VIP - name: {:?}. price: {:?}", name, price),
            TicketType::BackStage(name, price) => println!("BackStage - name: {:?}. price: {:?}", name, price),
            TicketType::Standard(price) => println!("Standard - price: {:?}", price),
        };
    }

}
