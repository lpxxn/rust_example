use z_2_0_2_trait::{NewsArticle, notify, notify2, notify3};

fn main() {
    let n = NewsArticle {
        head_line: String::from("big news"),
        content: String::from("jock")
    };
    notify(n);
    // notify2(&n);
    // notify3(&n);


    let v1 = vec![String::from("a"), String::from("word"), String::from("hi")];
    println!("{}", largest(&v1));
    println!("{}", largest2(&v1));
    println!("{}", largest3(&v1));
}

fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list.iter() {
        if largest < *item {
            largest = item.clone();
        }
    }
    largest
}

fn largest2<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if largest < item {
            largest = item
        }
    }
    largest
}


fn largest3<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list.iter() {
        if largest < *item {
            largest = item.clone();
        }
    }
    largest
}