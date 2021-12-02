// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Position {
    Maintenance,
    Marketting,
    Managers,
    LineSupers,
    Kitchen,
    AssemblyTechs,
}

enum EmploymentStatus {
    Employed,
    Terminated,
}

struct Employee {
    name: String,
    position: Position,
    status: EmploymentStatus,
}

fn set_access(employee: Employee) -> Result<(), String> {
    match employee.status {
        EmploymentStatus::Terminated => return Err("denied(terminated)".to_owned()),
        _ => ()
    };
    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Marketting => Ok(()),
        Position::Managers => Ok(()),
        _ => Err("denied".to_owned()),
    }
}

fn get_access(employee: Employee) -> Result<(), String> {
    set_access(employee)?;
    println!("ok");
    Ok(())
}

fn print_access(employees: Vec<Employee>) {
    for employee in employees {
        println!("+---------------------------+");
        println!("Name: {}", employee.name);
        print!("Access: ");

        match get_access(employee) {
            Err(e) => println!("{}", e),
            _ => (),
        };
    }
}

fn get_employees() -> Vec<Employee> {
    let victor = Employee {
        name: "Victor".to_owned(),
        position: Position::Maintenance,
        status: EmploymentStatus::Employed,
    };

    let greg = Employee {
        name: "Greg".to_owned(),
        position: Position::AssemblyTechs,
        status: EmploymentStatus::Employed,
    };

    let john = Employee {
        name: "John".to_owned(),
        position: Position::Managers,
        status: EmploymentStatus::Terminated,
    };

    let kathryn = Employee {
        name: "Kathryn".to_owned(),
        position: Position::LineSupers,
        status: EmploymentStatus::Employed,
    };

    let sarah = Employee {
        name: "Sarah".to_owned(),
        position: Position::Marketting,
        status: EmploymentStatus::Employed,
    };

    let tara = Employee {
        name: "Tara".to_owned(),
        position: Position::Kitchen,
        status: EmploymentStatus::Employed,
    };

    let employees = vec![ tara, sarah, kathryn, john, greg, victor ];

    return employees
}

fn main() {
    let employees = get_employees();
    print_access(employees);
}
