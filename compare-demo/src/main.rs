fn main() {
    println!("Hello, world!");
    let r1: f64 = 0.1 + 0.2;
    let r2: f64 = 0.3;
    let abc_r = (r2 - r1);
    println!("{}, abc_r: {}, abc: {}", r1 == r2, abc_r, abc_r.abs());
    println!("{}", abc_r.abs() <= f64::EPSILON);
}
// compare float
