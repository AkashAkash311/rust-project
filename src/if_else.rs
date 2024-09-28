
fn if_elsefn(){
    let x: i32 = 10;

    // if x==1 {
    //     println!("I'm x yk {}", x)
    // } else if x <= 10{
    //     println!("I'm else if yk {}", x)
    // } else {
    //     println!("I'm else yk {}", x)
    // }

    let result = if x==10{
        println!("I'm x yk {}", x)
    } else{
        println!("I'm else yk {}", x)
    };

    return result;

}

pub fn main_if_else(){
    if_elsefn();
}