
use std::io;
use std::fmt::Display;

#[derive(Debug)]
struct User {
    name: String,
    role: String,
    gender: String,
}  

fn main() {
println!("Input your full name:");
let mut full_name = String::new();
io::stdin().read_line(&mut full_name).expect("Invalid input");
full_name = full_name.trim.to_string();

println!("Input your role:");
let mut work_role = String::new();
io::stdin().read_line(&mut work_role).expect("Invalid input");
work_role = work_role.trim().to_string();

println!("Input your gender:");
let mut real_gender = String::new();
io::stdin().read_line(&mut real_gender).expect("Invalid input");
real_gender = real_gender.trim().to_string();

let user_info = User {
    name: full_name,
    role: work_role,
    gender: real_gender,
};

// let nom = user_info.name;
// let rol = user_info.role;
// let gend = user_info.gender;

println!("The details of the current user include {}:", user_info);


}

// fn input_user() {
    
// }