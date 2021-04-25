use alyx_uni_code::decode_from_str;
#[test]
fn decode_true_from_str() {
    let str = "true";
    let bool: bool = decode_from_str(str).unwrap();
    assert!(bool);
}

#[test]
fn decode_false_from_str() {
    let str = "false";
    let bool: bool = decode_from_str(str).unwrap();
    assert!(!bool);
}
