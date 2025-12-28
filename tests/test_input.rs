use day_7::*;
use std::fs;

#[test]
fn test_input() {
    let input = fs::read_to_string("input.txt").expect("Failed to access file.");

    let manifold = parse_manifold(input.to_string());
    let result   = trace_tachyon(manifold);

    assert_eq!(result, 21);
}
