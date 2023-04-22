#[cfg(test)]
mod tests {
    #[test]
    fn test_it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_it_doesnt_work() {
        panic!("haha!");
    }
}
