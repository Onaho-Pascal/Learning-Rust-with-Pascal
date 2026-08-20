use std::io;

fn main() {
    println!("You are to input a nitrogenous base of your choice:");
    let mut dna_base = String::new();

    io::stdin().read_line(&mut dna_base).expect("The input was invalid");
    let dna_base: char = dna_base.trim().parse().expect("The input was invalid at this stage");

    if dna_base == 'A' || dna_base == 'G'{
        println!("Purine Base");
    } else if dna_base == 'T' || dna_base == 'C' {
        println!("Pyrimidine");
    } else {
        println!("Not a nitrogenous base");
    }

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Input an array index of your choice:");
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Not a valid array");

    let index: usize = index.trim().parse().expect("Not a valid input");

    let final_number = a[index];
    println!("The number at index {} of array a is {}", index, final_number);
}