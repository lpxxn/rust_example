mod phrases;

fn main() {
    phrases::hello1();
    // 这个项目我们只能是这么访问
    phrases::greetings::hello2();
}
