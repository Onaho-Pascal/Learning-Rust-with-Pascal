fn main() {
    let x: (u32, char, f64) = (350, 'G', 7.5);
    // To access an element in tuples
    let nitro_base = x.1;
    println!("The nitrogenous base in the tuple is {}", nitro_base);

    // Array
    let nitrogenous_bases: [char; 5] = ['A', 'C', 'T', 'G', 'U'];
    // TO access an element in Array

    let adenine = nitrogenous_bases[0];
    let uracil = nitrogenous_bases[4];
    let thymine = nitrogenous_bases[2];

    println!("{} replaces {} to complement {} in RNA", uracil, thymine, adenine);


}