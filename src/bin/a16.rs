// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>
}

fn main() {
    let gene = Student {
        name: "Gene".to_owned(),
        locker: None,
    };
    let john = Student {
        name: "John".to_owned(),
        locker: Some(12),
    };
    let jena = Student {
        name: "Jena".to_owned(),
        locker: Some(15),
    };
    
    let students = vec![gene, john, jena];

    for student in students {
        println!("name: {:?}", student.name);
        match student.locker {
            Some(num) => println!("locker: {:?}", num),
            None => println!("no locker info"),
        }
    }
    
}
