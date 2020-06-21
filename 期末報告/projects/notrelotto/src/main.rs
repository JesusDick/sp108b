extern crate rand;

use rand::Rng;


fn main() {
    println!("welcome to lotto");
    const SIZE: usize = 5;
    let mut lotto = [0; SIZE];

    for i in 0..lotto.len() {
        lotto[i] = rand::thread_rng().gen_range(1, 50);
        for j in 0..i {
            if lotto[i] == lotto[j]{
                lotto[i] = rand::thread_rng().gen_range(1, 50);
            }
            
        }
    }
    for i in 0..lotto.len() {
        println!("{}", lotto[i]);
    } 

}
