pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn sub(left: u64, right: u64) -> u64 {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn sub_works() {
        let result = sub(2, 2);
        assert_eq!(0, result);
    }
}
