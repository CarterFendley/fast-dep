use fast_dep::minimal_parser;

#[test]
fn test_parser() {
    let file_contents = include_str!("res/test.py");

    minimal_parser::parse(&file_contents);
}