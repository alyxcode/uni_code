use alyx_uni_code::encode_to_string;

#[test]
fn encode_true_to_string() {
    let bool = true;
    let string = encode_to_string(&bool).unwrap();
    assert_eq!(string, "true".to_string());
}

#[test]
fn encode_false_to_string() {
    let bool = false;
    let string = encode_to_string(&bool).unwrap();
    assert_eq!(string, "false".to_string());
}
