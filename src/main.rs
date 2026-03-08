fn main() {
    let name = String::from("John Doe"); // memory allocation
    let name2 = name.clone(); // need to clone
    println!("Name1 is {}, and Name 2: {}", name, name2);
    
    let char1 = '1'; // stack allocation
    let char2 = char1; // no need to clone
    println!("char1 is {}, char2 is {}", char1, char2);
}