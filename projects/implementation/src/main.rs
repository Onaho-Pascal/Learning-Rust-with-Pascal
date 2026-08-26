use std::io;

struct Rectangle {
    width: f64,
    height: f64,
}

struct Dog {
    name: String,
    age: u8,
}

impl Dog {
    fn bark(&self) {
        println!("Say woof!, {}", self.name);
    }
}



fn main() {
    
    let caesar = Dog {name: String::from("Rex"), age: 10};

    rex.bark(); 



    println!("Input the value for the width:");
    let mut w = String::new();

    io::stdin().read_line(&mut w).expect("invalid input");
    let w: f64 = w.trim().parse().expect("invalid input");

    println!("Input the value for the height:");
    let mut h = String::new();

    io::stdin().read_line(&mut h).expect("Invalid input");
    let h: f64 = h.trim().parse().expect("Invalid input");



    let rect = Rectangle {

        width: w,
        height: h,

    };

    let final_area = area(&rect);

    println!("The area of the rectangle is {}", final_area);

}

fn area(r: &Rectangle) -> f64 {
    r.width * r.height
}
