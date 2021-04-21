// Topic: Multithreading
//
// Requirements:
// * Run expensive_1 and expensive_2 in separate threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish
// * Use the cheap() function as part of the message

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(500));
    "threads"
}

fn msg_excited() -> &'static str {
    "!"
}

fn main() {
    use std::thread;

    let msg_one = thread::spawn(move || msg_hello());
    let msg_two = thread::spawn(move || msg_thread());

    if let Ok(msg_one) = msg_one.join() {
        if let Ok(msg_two) = msg_two.join() {
            println!("{}{}{}", msg_one, msg_two, msg_excited());
        }
    }
}
