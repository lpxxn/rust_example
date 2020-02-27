智能指针是指一类数据结构，他们表现类似于指针，但有额外的元数据，他拥有一个引用计数。
引用计数记录智能指针总共有多少个所有者，当没有所有者时进行清除数据

普通 引用和智能指针区别是引用只是借用数据的指针，智能指针拥有他们指向的数据

智能指针使用结构体实现实现了Deref和Drop trait（类似于析构）
Defer trait 允许智能指针结构体实例表现的借引用一样 像解引用 int* p = ...  *p

Box<T>, 用于在堆上分配
Rc<T> 一个引用计数类型，数据可以有多个所有者
Ref<T> RefMut<T> 通过RefCall<T 访问>，一个在运行时，而不是在编译时执行借用无则的类型。


Box<T> is for single ownership.
Rc<T> is for multiple ownership.
Arc<T> is for multiple ownership, but threadsafe.
Cell<T> is for “interior mutability” for Copy types; that is, when you need to mutate something behind a &T.

// https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/


### Box<T>
Box<T> 只能有一个 ownership 
```
let x = Box::new(1);
let y = x;
// x no longer accessible here
```
Move/ownership semantics are not special to Box<T>; it is a feature of all types which are not Copy.

1. 内部可变性：允许在使用不可变引用时改变数据。
2. 通过RefCell<T>在运行时检查借用规则（通常情况下，是在编译时检查借用规则），RefCell<T>代表其数据的
   唯一所有权。
3. 类似于Rc<T>, RefCell<T>只能用于单线程场景。

Rc<T> 允许一个数据有多个所有者；Box<T>和RefCell<T> 有单一所有者
Box<T> 允许在编译时执行不可变或者可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；
RefCell<T> 允许在运行时执行不可变或可变检查。
因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便RefCell<T>自身是不可变的情况下改变
其内部的

RefCell<T>/Rc<T> 与 Mutex<T>/Arc<T>
Mutex<T> 提供内部可变性，类似于RefCell
RefCell<T>/Rc<T>是非线程安全的，Mutex<T>/Arc<T>是线程安全

## Cell<T>
Cell<T> is a type that provides zero-cost interior mutability, but only for Copy types. Since the compiler knows that all the data owned by the contained value is on the stack, there’s no worry of leaking any data behind references (or worse!) by simply replacing the data.
```
let x = Cell::new(1);
let y = &x;
let z = &x;
x.set(2);
y.set(3);
z.set(4);
println!("{}", x.get());
```
This has the same runtime cost as the following:
```
let mut x = 1;
let y = &mut x;
let z = &mut x;
x = 2;
*y = 3;
*z = 4;
println!("{}", x;
```
## RefCell<T>
RefCell<T> also provides interior mutability, but isn’t restricted to Copy types.
```
let x = RefCell::new(vec![1,2,3,4]);
{
    println!("{:?}", *x.borrow())
}

{
    let my_ref = x.borrow_mut();
    my_ref.push(1);
}
```

## Synchronous types
## Arc<T>
Arc<T> is just a version of Rc<T> that uses an atomic reference count (hence, “Arc”). This can be sent freely between threads.

## Mutex<T> and RwLock<T>
```
{
    let guard = mutex.lock();
    // guard dereferences mutably to the inner type
    *guard += 1;
} // lock released when destructor runs

```

## Weak
Weak用于不拥有所有权的引用,通过调用upgrade调用值,这个方法返回一个Optical<Rc<T>>
Weak不能阻止值被丢弃,当值被丢弃时,调用upgrade时返回None
使用Weak可以避免Rc的循环引用
调用Rc::downgrade来获得一个Weak指针

```
use std::rc::Rc;
 
fn main() {
    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    println!("{}", strong_five.unwrap());
    println!("weak: {}, strong: {}", Rc::weak_count(&five), Rc::strong_count(&five));
}

```




