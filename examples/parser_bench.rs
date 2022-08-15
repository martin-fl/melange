use melange::parser::Parse;

fn main() {
    let input = "type Point := | Circle (A~Point, f64) | Triangle (A~Point, A~B~Point, root~C~Point) | None.";
    let mut stream = melange::lexer::Scanner::new(&input).stream();
    println!("{}", melange::ast::TyDef::parse(&mut stream).unwrap());
}
