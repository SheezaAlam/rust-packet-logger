#[cfg(test)]
mod tests {
    // A very simple test
    #[test]
    fn test_addition() {
        let result = 2 + 3;
        assert_eq!(result, 5); // passes ✅
    }

    // Another test
    #[test]
    fn test_subtraction() {
        let result = 10 - 4;
        assert_eq!(result, 6); // passes ✅
    }
}

/* output
test tests::tests::test_addition ... ok
test tests::tests::test_subtraction ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
 */