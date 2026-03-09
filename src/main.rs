fn main() {
    let name = String::from("John Doe"); // memory allocation
    let name2 = name.clone(); // need to clone
    println!("Name1 is {}, and Name 2: {}", name, name2);

    let char1 = '1'; // stack allocation
    let char2 = char1; // no need to clone
    println!("char1 is {}, char2 is {}", char1, char2);
    print_message(&char2);

    let tuple = ("John Doe", "John Doe");
    let (john, doe) = tuple;
    print!("Tuple is {}, and {} ", john, doe);

    let items = [1,2,3,4,5];
    println!("items: {:?}", items);

    for item in &items {
        println!("Current item is: {} ", item);
    }

    let mut idx = 0;
    while idx < items.len() {
        println!("At while loop: {}", items[idx]);
        idx += 1;
    }
    
    let name = String::from("John Doe");
    assign(&name);
    
    if name == "John Doe" {};
    
    let result = String::from("Result 1");
    let modified_result = modify_result(result);
    println!("Modified result: {}", modified_result);
}

fn print_message(value: &char) -> &char {
    println!("{}", value);
    return value;
}

fn assign(value: &String) {
    let copied = value;
    println!("Println Copied: {}", copied);
}

fn modify_result(mut value: String) -> String {
    let value = value + " [MODIFIED]";
    return value;
}