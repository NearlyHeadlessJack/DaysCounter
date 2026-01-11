use days_counter::calculator;

#[test]
fn test_calculator() {
    assert_eq!(calculator((2026, 1, 11), (2027, 1, 1)), 355);
    assert_eq!(calculator((2026, 1, 11), (2026, 1, 11)), 0);
    assert_eq!(calculator((1999, 1, 11), (2021, 3, 1)), 8085);
    assert_eq!(calculator((1999, 1, 11), (2041, 3, 1)), 15390);
    assert_eq!(calculator((1999, 1, 11), (2041, 4, 1)), 15421);
    assert_eq!(calculator((1999, 1, 11), (3999, 4, 1)), 730565);
    assert_eq!(calculator((2000, 1, 1), (2999, 1, 1)), 365242);
}
