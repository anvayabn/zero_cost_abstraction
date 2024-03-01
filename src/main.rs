use zero_cost_abs_bm::{no_abstraction, with_abstraction};

fn main() {

    let vec: Vec<i32> = vec![1,2,3,4,5,6];

    println!("No Abstraction {}", no_abstraction(vec.clone()));
    println!("With Abstraction {}", with_abstraction(vec.clone()));

}
