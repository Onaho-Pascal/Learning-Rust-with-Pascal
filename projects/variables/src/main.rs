fn main() {
    // let mut x = 24;

    // println!("The value of the first variable is {}", x);
    //x = 15;
    // let mut y = 40;
    // let z = 10;
    // x = 15;
    //println!("The value of the secon variable is {}", x);
    let read_count = 150000; // raw reads (you do not want to change or alter)
    let mut filtered_count = read_count; // created a clone of the original
    println!("This is the first version of the filtered count: {}", filtered_count);
    filtered_count = read_count / 50;
    println!("The value of the raw count is {}", read_count);
    println!("The value of the filtered count is {}", filtered_count);
    
}
