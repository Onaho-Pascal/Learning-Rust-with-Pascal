use std::io;

fn main() {
    println!("You are to input a nitrogenous base of your choice:");
    let mut dna_base = String::new();

    io::stdin().read_line(&mut dna_base).expect("The input was invalid");
    let dna_base: char = dna_base.trim().chars().next().expect("The input was invalid at this stage");

    if dna_base == 'A' {
        println!("Adenine");
    } else if dna_base == 'G' {
        println!("Guanine");
    } else if dna_base == 'T' {
        println!("Thymine");
    } else if dna_base == 'C' {
        println!("Cytosine");
    } else {
        println!("Not a nitrogenous base");
    }

}