mod funtions;
mod if_else;


fn main() {
    let x: i32 = 500;
    let y: i32 = 50000;
    let z = 500.003;

    println!("{}, {}, {}", x, y, z);

    funtions::my_function();
    if_else::main_if_else();
}
