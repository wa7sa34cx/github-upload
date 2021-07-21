trait Foo {
    fn foo() -> String;
}

trait Bar {
    fn bar(&self) -> String;
    fn plus_one(self) -> Self;
}

enum AppError {
    One,
    Tow,
    Three,
}

impl Foo for AppError {
    fn foo() -> String {
        String::from("foo")
    }
}

impl Bar for u32 {
    fn bar(&self) -> String {
        String::from("bar")
    }
    fn plus_one(self) -> Self {
        self + 1
    }
}

fn main() {
    println!("Hello, world!");

    let foo = AppError::foo();
    println!("{}", foo);

    let x: u32 = 5;
    println!("{}", x.bar());
    println!("{}", x.plus_one());

    // ---

    let x = 6;
    assert_eq!(-x, -6);
    assert_eq!(!x, -7);

    // ---
    let mut vals = vec![2, 3, 1, 2, 2];
    while let Some(v @ 1) | Some(v @ 2) = vals.pop() {
        // Prints 2, 2, then 1
        println!("{}", v);
    }

    // ---
    let x = 10;

    let message = match x {
        1 | 2 => "not many",
        2..=7 => "a few",
        _ => "a lot of",
    };
    println!("There are {} apples", message);

    // ---
    println!();

    let numbers: Vec<i32> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    ];

    // let even_numbers = numbers
    //     .into_iter()
    //     .filter(|n| n % 2 == 0)
    //     .collect::<Vec<_>>();

    let even_numbers: Vec<_> = numbers
        .iter()
        .cloned()
        .filter(|n| n % 2 == 0)
        .collect();

    println!("{:?}", even_numbers);
    println!("{:?}", numbers);
}
