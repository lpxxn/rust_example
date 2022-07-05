#[test]
fn it_works() {
    let s1 = Some("some1");
    let s2 = Some("some2");
    let n: Option<&str> = None;

    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");

    println!("or: {:?}", s1.or(s2) == s1);
    println!("or: {:?}", n.or(s2) == s2);
    println!("and: {:?}", s1.and(s2) == s2);

    println!("or: {:?}", o1.or(o2) == o1);
    println!("or: {:?}", o1.or(o2) == o1);
}
