// 这个项目不能 mod greetings
// 不能直接把子文件夹里的 mod 引入
// structure_demo_2 的可以
mod phrases;

fn main() {
    phrases::hello1();
    // 所以只能是这么访问
    phrases::greetings::hello2();
}
