
fn for_loop(){


    let mut counter: i32 = 0;
    
    for i in  0..100 {
        // counter += 1;
        counter = counter+i;
        println!("{}", counter)
    }

}

fn loooooop(){
    let mut counter: i8 = 12;

    loop {
        counter += 1;
        if counter == 100 {
            println!("{}", counter);
            break;
        }
    }
}

fn whileeeeeeeeeeee(){
    let mut count: i32 = 0;

    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }
}


pub fn loops_main(){
    for_loop();
    loooooop();
    whileeeeeeeeeeee();
}