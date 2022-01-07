// BACKTRACE=1
/*
Result<T, E>
enum Resutl<T, E> {
    Ok(T),
    Err(E),
}
*/
use std::io;
use std::io::Read;
use std::fs::File;
use std::ops;
fn main() {
    // let f = File::open("hello.txt");

    // let r = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("error: {:?}", error),
    // };

    // 也可以这样
    let f = File::open("hello.txt").unwrap();
    
    // 这样
    //let f = File::open("hello.txt").expect("abdef");

    println!("Hello, world!");

    //panic!("crash here");

    let r = read_username_from_file3();
    match r {
        Ok(s) => println!("s {}", s),
        Err(e) => println!("err {:?}", e),
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        // 提前return error
        Err(error) => return Err(error),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    } // 这里没有 ;
}

fn read_username_from_file2() -> Result<String, io::Error> {
    // ? 是如果有错误就直接返回 Error
    /*
    把 result 用 match 连接起来会显得很难看；幸运的是，? 运算符可以把这种逻辑变得 干净漂亮。
    ? 运算符用在返回值为 Result 的表达式后面，它等同于这样一个匹配 表达式：
    其中 Err(err) 分支展开成提前返回的 return Err(err)，而 Ok(ok) 分支展开成 ok 表达式。
    https://rustwiki.org/zh-CN/rust-by-example/std/result/question_mark.html
    */
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
} 

/*
给 Result 取别名
当我们要重用某个 Result 类型时，该怎么办呢？回忆一下，Rust 允许我们 创建别名。若某个 Result 有可能被重用，我们可以方便地给它取一个别名。

在模块的层面上创建别名特别有帮助。同一模块中的错误常常会有相同的 Err 类 型，所以单个别名就能简便地定义所有相关的 Result。这太有用了，以至于标准库 也提供了一个别名： io::Result！

下面给出一个简短的示例来展示语法：
```
use std::num::ParseIntError;

// 为带有错误类型 `ParseIntError` 的 `Result` 定义一个泛型别名。
type AliasedResult<T> = Result<T, ParseIntError>;

// 使用上面定义过的别名来表示上一节中的 `Result<i32,ParseIntError>` 类型。
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// 在这里使用别名又让我们节省了一些代码量。
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}

```


*/