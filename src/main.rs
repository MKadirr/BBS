use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

fn main() {
    println!("Hello World!");
}

#[test]
fn calculator1() {
    assert!(grammar::TermParser::new().parse("22").is_ok());
    assert!(grammar::TermParser::new().parse("(22)").is_ok());
    assert!(grammar::TermParser::new().parse("((((22))))").is_ok());
    assert!(grammar::TermParser::new().parse("((22)").is_err());
}
