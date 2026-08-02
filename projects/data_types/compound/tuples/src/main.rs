fn main() {
//             variant position, reference allele, alternate allele
 let variant: (u32, char, char) = (156897, 'A', 'G');

 // let (var_pos, ref_all, alt_all) = variant;

// println!("The variant position is {}", var_pos);
// println!("The reference allele is {}", ref_all);
// println!("The alternate allele is {}", alt_all);

 let var = variant.0;
 let refr = variant.1;
 let alt = variant.2;

 println!("The variant position is {}", var);
 println!("The reference allele is {}", refr);
 println!("The alternate allele is {}", alt);




}