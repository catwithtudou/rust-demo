use rand::Rng;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn rand_number() -> i32 {
    rand::thread_rng().gen_range(1..101)
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_eq!(3, add_one(2));
        assert!(rand_number() < 200, "x should be less than 200")
    }
}
