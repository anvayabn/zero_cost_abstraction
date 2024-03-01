pub fn no_abstraction(vec: Vec<i32>) -> i32 {

    let mut sum = 0 ;

    for i in 0..vec.len(){

        sum += vec[i]*vec[i];
    }

    sum

}

pub fn with_abstraction(vec: Vec<i32>) -> i32 {

    let sum = vec.iter().map(|&x| x*x).sum();

    sum
}
