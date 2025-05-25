use dlna_dmr::add;

#[test]
fn test_add() {
    let left = 2;
    let right = 2;
    let result = add(left, right);
    assert_eq!(result, 4);
}
