trait Hello {
    // 默认实现 + 无Override
    fn say_hi(&self) {
        println!("hi")
    }
    // 默认实现 + Override
    fn say_hello(&self) {
        println!("hello")
    }
    // 无默认实现
    fn say_nothing(&self);
}

struct Student {}
impl Hello for Student {
    fn say_nothing(&self) {
        println!("impl say_nothing for student")
    }
}

struct Teacher {}
impl Hello for Teacher {
    // override
    fn say_hello(&self) {
        println!("I'm teacher Lee.")
    }
    fn say_nothing(&self) {
        println!("impl say_nothing for teacher")
    }
}

fn as_return() -> impl Hello {
    Student {}
}

pub fn demo() {
    let s = Student {};
    s.say_hi();
    s.say_hello();
    s.say_nothing();
    s.hello(32);

    let t = Teacher {};
    t.say_hi();
    t.say_hello();
    t.say_nothing();

    as_return().say_hello();

    println!("trait demo2...........");
    demo2();

    println!("trait demo3...........");
    demo4()
}

// 这个就是孤儿规则在起作用：
// 当你为某类型实现某 trait 的时候，
// 该类型或者trait至少有一个是在当前 crate 中定义的，
// 你不能为第三方的类型实现第三方的 trait 。
// use std::ops::Add;
// impl Add<i32> for i32 {}

trait GenericTrait<T> {
    fn hello(&self, i: T);
}

impl GenericTrait<i32> for Student {
    fn hello(&self, i: i32) {
        println!("impl GenernicTrait for Student: {}", i)
    }
}

// trait 作用一
// 抽象接口，如上使用

// trait 作用二
// 泛型约束
trait Run {
    fn running(&self);
}
trait Eat {
    fn eating(&self);
}

#[derive(Debug)]
struct Horse {}
impl Run for Horse {
    fn running(&self) {
        println!("running");
    }
}

impl Eat for Horse {
    fn eating(&self) {
        println!("eating");
    }
}

fn demo_constraint<T: Run + Eat>(t: T) -> T {
    println!("demo_constraint...");
    t.running();
    t.eating();

    t
}

fn demo_constraint2<T>(x: T)
where
    T: Run + Eat,
{
    println!("demo_constraint2...");
    x.running();
    x.eating();
}

fn demo2() {
    let h = Horse {};
    let h = demo_constraint(h);
    demo_constraint2(h);
}

trait Live: Run + Eat {}

impl Live for Horse {}

fn as_return_live() -> impl Live {
    Horse {}
}

fn demo3() {
    as_return_live().eating();
    as_return_live().running();
}

/*
编译器允许你通过 #[derive] 属性自动实现一些Trait，这些Trait包含：
比较相关的：Eq，PartialEq，Ord，PartialOrd
Clone，经由&T创建T
Copy，实现T的复制语义
Hash，计算&T的哈希值
Default，创建数据类型的空实例
Debug，使得可以用 {:?}格式化T
*/
#[derive(Debug, Clone)]
struct Animal {}

// unsafe trait

unsafe impl Send for Student {}

// 常见的Trait

// a. Deref，DerefMut ： 可以用来重载 *操作符，也可以用来 method resolution 以及 deref coercions（解引用转换）

// b. Drop： 用于解构，在一个类型的变量被销毁前执行

// c. Copy ：如果类型实现了这个Trait，则该类型的值会使用拷贝语义替代移动语义，并避免所有权的变更。一个类型实现Copy有两个前提条件：1、这个类型不能实现Drop；2、这个类型的所有字段都必须为Copy的。

// 编译器会为以下类型自动实现Copy ，除此之外的类型如果想要impl Copy必须先impl Clone：

//   Numeric typeschar，bool 以及 !由Copy类型组成的Tuples由Copy类型组成的ArraysShared references（共享引用/借用）Raw pointers（裸指针）

// d. Clone：Copy的超集，它也是需要编译器生成implementations，编译器会为以下类型自动实现Clone：

//   内置实现了Copy的类型由Clone类型组成的Tuples由Clone类型组成的Arrays

// e. Send：表明一个类型的值是否可以安全的在线程间传递。

// f. Sync：表明一个类型的值是否可以安全的在线程间共享。

// g. Sized：编译器可确定大小的类型

// i. Unsize：编译器无法确定大小的类型

////////////////////////////////////////////////////
// 消除Trait歧义（Disambiguating overlapping traits）
trait One {
    fn action(&self) {
        println!("One::action")
    }
}

trait Two {
    fn action(&self) {
        println!("Two::action")
    }
}

impl One for Student {}
impl Two for Student {}

fn demo4() {
    let s = Student {};
    // s.action(); // error: multiple `action` found
    <Student as One>::action(&s);
    <Student as Two>::action(&s);
}
