#[test]
fn testI1() {
    let arr = [1, 2, 3];
    let mut arr_iter = arr.iter();
    println!("{:?}", arr_iter.next());
    println!("{:?}", arr_iter.next());
    println!("{:?}", arr_iter.next());
    println!("{:?}", arr_iter.next());

    let mut arr_iter = arr.iter();
    let mut arr_iter = arr_iter.skip(2);
    println!("{:?}", arr_iter.next());
    println!("{:?}", arr_iter.next());
}
/*
在之前的代码中，我们统一使用了 `into_iter` 的方式将数组转化为迭代器，除此之外，还有 `iter` 和 `iter_mut`，聪明的读者应该大概能猜到这三者的区别：
- `into_iter` 会夺走所有权
- `iter` 是借用
- `iter_mut` 是可变借用
其实如果以后见多识广了，你会发现这种问题一眼就能看穿，`into_` 之类的，都是拿走所有权，`_mut` 之类的都是可变借用，剩下的就是不可变借用。

 */
#[test]
fn testI2() {
    let arr :[i32; 3] = [1, 2, 3];
    let arr2 :[i32; 3]= [3, 4, 5];
    let arr3 :Vec<_> = arr.into_iter().zip(arr2.into_iter()).collect();
    println!("{:?}", arr3);

    let x = [1, 2, 3];
    let y = [4, 5, 6];

    let z: Vec<_> = x
        .iter()
        .zip(y.iter())
        .map(|(a, b)| a + b)
        .collect();
    println!("2: {:?}", z);

    let z= x.iter().fold(10, |sum, acm| sum + acm);
    println!("fold: {}", z);

    let values = vec![1, 2, 3];
    for v in values.into_iter() {
        println!("{}",v)
    }
}