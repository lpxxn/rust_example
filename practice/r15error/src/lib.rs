use std::fmt;
use std::fmt::{Formatter, write};

struct AppError {
    code: usize,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let err_msg = match self.code {
            404 => "sorry, can not find to page!",
            _ => "sorry, something is wrong! please try again!",
        };
        write!(f, "{}", err_msg)
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "appErr {{ code: {}, message: {} }}", self.code, self.message)
    }
}

fn produce_error() -> Result<(), AppError> {
    Err(AppError {
        code: 404,
        message: "page not found".to_string(),
    })
}

#[test]
fn it_works() {
    match produce_error() {
        Err(e) => eprintln!("{}", e),
        _ => println!("no error")
    }
    let a: i32= (1,).0;
    println!("{}", a);

}
