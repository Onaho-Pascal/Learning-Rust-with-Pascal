
// fn new_function(x: u8) {
    //println!("An example of a nitrogenous base in a DNA or RNA molecule is {}.", x);
//}
fn dna_length(x: u64) -> u64 {
    x * 500
}

fn main() {

let total_length = dna_length(50);
println!("The total length of the DNA sequence is {}bp", total_length);

}

