
pub fn get10() -> i32 {
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get10() {
        assert_eq!(get10(), 10);
    }
}
