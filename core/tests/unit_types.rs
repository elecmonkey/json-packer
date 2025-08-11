use json_packer::test_expose::tag;

#[test]
fn type_tags_values() {
    assert_eq!(tag::NULL, 0b000);
    assert_eq!(tag::BOOL_FALSE, 0b001);
    assert_eq!(tag::BOOL_TRUE, 0b010);
    assert_eq!(tag::INT, 0b011);
    assert_eq!(tag::FLOAT, 0b100);
    assert_eq!(tag::STRING, 0b101);
    assert_eq!(tag::OBJECT, 0b110);
    assert_eq!(tag::ARRAY, 0b111);
}
