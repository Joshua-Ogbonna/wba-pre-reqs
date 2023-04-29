pub fn run() {
    let age = 29;
    let check_id = true;
    let knows_person_of_age = true;

    if age >= 18 && check_id || knows_person_of_age {
        println!("Hello, you're qualified bro");
    } else if age < 18 && check_id {
        println!("You have to leave now or I call the Police")
    } else {
        println!("Hey man, your ID please")
    }
}

// Conditionals used to check for truthy and falsy statements and run stuff based on the conditions