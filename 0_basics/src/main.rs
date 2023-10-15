use crate::LinkedList::{Cons, Nil};
use std::fmt::{format, Formatter};
use std::{fmt, mem};

// Debug позволяет выводить структуру в print добавляя код для вывода через impl
#[derive(Debug)]
struct TEST(i64, i64);

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

struct CustomList(Vec<i8>);

// Мы можем кастомизировать вывод сделав свою реализацию вывода (fmt::Display)
impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Display for CustomList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vertor = &self.0;
        write!(f, "[")?;

        for (count, value) in vertor.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, value)?;
        }

        write!(f, "]")
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// RGB в HEX это очень легко оказывается, лол)
impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for (_, value) in vec![self.red, self.green, self.blue].iter().enumerate() {
            if value == &0u8 {
                write!(f, "00")?;
                continue;
            }
            if value < &17u8 {
                write!(f, "0{:X}", value)?;
                continue;
            }
            write!(f, "{:X}", value)?
        }
        write!(f, "")
    }
}

// В Rust можно объединять данные в скобках
#[derive(Debug)]
struct Matrix(f64, f64, f64, f64);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "( {}, {} )\n", self.0, self.1)?;
        write!(f, "( {}, {} )", self.2, self.3)
    }
}

fn transpose(data: Matrix) -> Matrix {
    Matrix(data.0, data.2, data.1, data.3)
}

enum OperationStatus {
    Done,
    Error,
    Processed,
}

impl OperationStatus {
    fn exec(&self) {
        match self {
            OperationStatus::Done => {
                println!("DONE")
            }
            OperationStatus::Error => {
                println!("ERROR")
            }
            OperationStatus::Processed => {
                println!("IN PROCESS")
            }
        }
    }
}

enum Colored {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum LinkedList {
    Cons(u32, Box<LinkedList>),
    Nil,
}

impl LinkedList {
    fn new() -> LinkedList {
        Nil
    }

    fn prepend(self, v: u32) -> LinkedList {
        Cons(v, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{} {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

static mut LULZ: &str = "Lulz";

enum FooBar {
    Foo,
    Bar,
    FooBar(u8),
}

fn main() {
    println!("################################################");
    println!("{:?}", TEST(10, 20));
    println!("{}", Point { x: 10, y: 20 });
    println!("{}", CustomList(vec![10, 5, 13, 92]));
    println!(
        "{}",
        Color {
            red: 10,
            green: 14,
            blue: 99
        }
    );
    println!("################################################");
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    println!("################################################");
    println!("{}", Matrix(1.1, 1.2, 2.1, 2.2));
    println!();
    println!("{}", transpose(Matrix(1.1, 1.2, 2.1, 2.2)));
    println!("################################################");

    // Массивы определяются через указание типа и количества в квадратных скобках
    let test_array: [u8; 5] = [0, 3, 4, 5, 6];
    println!("size of array: {}", mem::size_of_val(&test_array));

    let slice: [i8; 10] = [10, 12, 42, 5, 56, 4, 57, 6, 32, 31];

    for index in 0..slice.len() {
        match slice.get(index) {
            Some(value) => print!("{:?}", value),
            None => {
                println!("\nIndex {} is unreachable", index);
                continue;
            }
        }
        if index != 0 {
            print!(",")
        }
    }
    println!();
    println!("################################################");
    OperationStatus::Done.exec();
    OperationStatus::Error.exec();
    OperationStatus::Processed.exec();
    println!("################################################");
    println!("roses are #{:06x}", Colored::Red as i32);
    println!("violets are #{:06x}", Colored::Blue as i32);
    println!("################################################");
    let mut custom_list = LinkedList::new();
    custom_list = custom_list.prepend(90);
    custom_list = custom_list.prepend(20);
    custom_list = custom_list.prepend(10);
    println!(
        "List len is {}. \nList: {}",
        custom_list.len(),
        custom_list.stringify()
    );
    println!("################################################");
    unsafe {
        println!("{}", LULZ);
        LULZ = "TESTT";
        println!("{}", LULZ);
    }
    println!("################################################");
    println!("{}", 78.42_f32 as u8 as char);
    println!("################################################");
    let testString = "013411".parse::<u32>().unwrap_or(0u32);
    println!("{}", testString);
    println!("################################################");
    let message: &str = 'neko0: loop {
        println!("Anime is aesthetic");
        'neko: loop {
            println!("But it's better if you had a person which you love");
            break 'neko0 "Sometimes you need that One Desire";
        }
    };
    println!("{}\nhttps://www.youtube.com/watch?v=prEQUMn-lJ8", message);
    println!("################################################");
    let c = FooBar::FooBar(7);
    if let FooBar::FooBar(test @ 1..=10) = c {
        println!("TEST {}", test);
    }
    println!("################################################");
}
