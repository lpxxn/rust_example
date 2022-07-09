
#[derive(Debug)]
#[derive(Copy, Clone)]
enum TestEnum {
    //RETYPE(String),
    //        ------ this field does not implement `Copy`
    RETYPE2(u32),
}

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
