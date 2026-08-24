fn main() {
    let mut counter = 0;

    let mut result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 5;
        }
    };
    println!("The result of the loop control flow is {}", result);
    result = result / 10;
    println!("But the result AFTER the loop control is {}", result);
    

}
