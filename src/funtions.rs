


// pub fn main() {
//     println!("Hello from my_function!");
// }


#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

pub fn strrrrrrutttt() {
    let person: Person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{:?}", person); // Output: Person { name: "Alice", age: 30 }
    
    println!("{:#?}", person); 
    /*
    Output:
    Person {
        name: "Alice",
        age: 30,
    }
    */
}
