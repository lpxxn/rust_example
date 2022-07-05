use std::{fmt, io};
use std::fmt::{Formatter, write};
use std::fs::File;
use std::path::is_separator;

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

impl From<io::Error> for AppError{
    fn from(e: io::Error) -> Self {
        AppError{
            code: 111,
            message: format!("appErr: {}", e.to_string())
        }
    }
}

#[test]
fn it_works() {
    match produce_error() {
        Err(e) => eprintln!("{}", e),
        _ => println!("no error")
    }
    eprintln!("{:?}", produce_error());
    eprintln!("{:#?}", produce_error());
}

#[test]
fn test_from_err() -> Result<(), AppError> {
    let _file = File::open("hahahahhahah")?;
    Ok(())
    // 上面的代码中除了实现 `From` 外，还有一点特别重要，那就是 `?` 可以将错误进行隐式的强制转换：`File::open` 返回的是 `std::io::Error`， 我们并没有进行任何显式的转换，它就能自动变成 `AppError` ，这就是 `?` 的强大之处！
}
