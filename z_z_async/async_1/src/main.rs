use futures::executor::block_on;
fn main() {
    let future = print_async();
    println!("Hello, world!");
    block_on(future);
}
/*
An async fn foo(args..) -> T is a function of the type fn(args..) -> impl Future<Output = T>. 
The return type is an anonymous type generated by the compiler.
*/
async fn print_async() {
    println!("hello from print_async");
}

/*
      async fn compute(){
          println!("async->");
      }

      fn compute()->impl Future<Output = ()>{
          println!("async->");
      }

*/