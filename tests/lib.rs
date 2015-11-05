extern crate dwarf;

use dwarf::parser::parse_dwarf_string;

#[test]
fn it_works() {
    assert_eq!(parse_dwarf_string(&b"2 + 2"[..]).unwrap(), 4);
}
