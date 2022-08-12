use lexer::Scanner;

mod interner;
mod lexer;
mod span;
mod token;

fn main() {
    let input = "\
type Point := (x y: f64).

type bool :=
    | true
    | false.

type Shape :=
    | Circle (Point, f64)
    | Triangle (Point, Point, Point)
    | Square (Point, Point).

type Orientation :=
    | Direct
    | Indirect.

fun turn_triangle (s: Shape) (o: Orientation) : Shape :=
    match (s, o) with
    | (Shape~Triangle a b c, Orientation~Direct) => Shape~Triangle c a b
    | (Shape~Triangle a b c, Orientation~InDirect) => Shape~Triangle b c a
    | _ => s
    end.";

    let mega_input = input.repeat(1000000);
    let mut scanner = Scanner::new(&mega_input);
    let now = std::time::Instant::now();
    let tokens = scanner.stream().collect::<Vec<_>>();
    let time = now.elapsed();
    println!(
        "took {}s for {}Mb ({} tokens) ({} Mb/s)",
        time.as_secs_f64(),
        mega_input.as_bytes().len() / 1024 / 1024,
        tokens.len(),
        mega_input.as_bytes().len() as f64 / 1024.0 / 1024.0 / time.as_secs_f64(),
    );

    // let mut scanner = Scanner::new(&input);
    // for token in scanner.stream() {
    //     println!(
    //         "{:?} -> \"{}\" =? \"{}\"",
    //         &token,
    //         &input[token.span().start()..][..token.span().len()],
    //         token.repr()
    //     )
    // }
}
