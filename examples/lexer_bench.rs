fn main() {
    let input = include_str!("points.l1").repeat(1000000);
    let input_size_mb = input.as_bytes().len() / 1024 / 1024;
    let scanner = melange::lexer::Scanner::new(&input);
    let start = std::time::Instant::now();
    let _ = scanner.stream().for_each(|_| {});
    let time = start.elapsed().as_secs_f64();
    println!(
        "took {}s to process {}Mb ({} Mb/s)",
        time,
        input_size_mb,
        input_size_mb as f64 / time
    );
}
