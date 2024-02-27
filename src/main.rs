// Variables
// Binding and mutability
// Numbers

fn main() {
    ex1();
    ex2();
    ex3();
    ex4();
    ex5();
    ex6();
    ex7();
    ex8();
    ex9();
    ex10();
    ex11();
    ex12();
    ex13();
    ex14();
    ex15();
    ex16();
    ex17();
    ex18();
    println!("Success!")
}

// Ex 1. *******************************************************************************

// // Fix the error below with least amount of modification to the code
// fn main() {
//     let x: i32; // Uninitialized but used, ERROR !
//     let y: i32; // Uninitialized but also unused, only a Warning !

//     assert_eq!(x, 5);
//     println!("Success!");
// }

fn ex1() {
    let x: i32 = 5;
    let _y: i32;

    assert_eq!(x, 5);
}

// Ex 2. *******************************************************************************

// Fill the blanks in the code to make it compile
// fn main() {
//     let __ __ = 1;
//     __ += 2;

//     assert_eq!(x, 3);
//     println!("Success!");
// }

fn ex2() {
    let mut x: i32 = 1;
    x += 2;

    assert_eq!(x, 3);
}

// Ex 3. *******************************************************************************

// // Fix the error below with least amount of modification
// fn main() {
//     let x: i32 = 10;
//     {
//         let y: i32 = 5;
//         println!("The value of x is {} and value of y is {}", x, y);
//     }
//     println!("The value of x is {} and value of y is {}", x, y);
// }

fn ex3() {
    let x: i32 = 10;
    let y: i32 = 10;
    {
        println!("x = {}, y = {} ", x, y)
    }

    println!("x = {}, y = {} ", x, y)
}

// Ex 4. *******************************************************************************

// // Fix the error with the use of define_x
// fn main() {
//     println!("{}, world", x);
// }

// fn define_x() {
//     let x = "hello";
// }

fn ex4() {
    define_x();
}

fn define_x() {
    let x: &str = "hello";
    println!("{}, world", x);
}

// Ex 5. *******************************************************************************

// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 5);
//     }

//     assert_eq!(x, 12);

//     let x = 42;
//     println!("{}", x); // Prints "42".
// }

fn ex5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x);
}

// Ex 6. *******************************************************************************

// // Remove a line in the code to make it compile
// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // Shadowing and re-binding
//     let x = x;
//     x += 3;

//     let y = 4;
//     // Shadowing
//     let y = "I can also be bound to text!";

//     println!("Success!");
// }

fn ex6() {
    let mut _x: i32 = 1;
    _x = 7;
    _x += 3;

    let _y: i32 = 4;
    // Shadowing
    let _y: &str = "I can also be bound to text!";
}

// Ex 7. *******************************************************************************

// Fix the warning below
// fn main() {
//     let x = 1;
// }
// Warning: unused variable: `x`

fn ex7() {
    //#[allow(unused_variables)]
    let _x: i32 = 1;
}

// Ex 8. *******************************************************************************

// // Fix the error below with least amount of modification
// fn main() {
//     let (x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);

//     println!("Success!");
// }

fn ex8() {
    let (mut x, y): (i32, i32) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);
}

// Ex 9. *******************************************************************************

// fn main() {
//     let (x, y);
//     (x,..) = (3, 4);
//     [.., y] = [1, 2];
//     // Fill the blank to make the code work
//     assert_eq!([x,y], __);

//     println!("Success!");
// }

fn ex9() {
    let (x, y): (i32, i32);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];

    assert_eq!([x, y], [3, 2]);
}

// Ex 10. *******************************************************************************

// // Remove something to make it work
// fn main() {
//     let x: i32 = 5;
//     let mut y: u32 = 5;

//     y = x;

//     let z = 10; // Type of z ?

//     println!("Success!");
// }

fn ex10() {
    let x: i32 = 5;
    let mut _y = 5;
    _y = x;
    let _z: i32 = 10;
}

// Ex 11. *******************************************************************************

// // Fill the blank
// fn main() {
//     let v: u16 = 38_u8 as __;

//     println!("Success!");
// }

fn ex11() {
    let _v: u16 = 38_u8 as u16;
}

// Ex 12. *******************************************************************************

// // Modify `assert_eq!` to make it work
// fn main() {
//     let x = 5;
//     assert_eq!("u32".to_string(), type_of(&x));

//     println!("Success!");
// }

// // Get the type of given variable, return a string representation of the type:
// // e.g "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn ex12() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));
}

// Ex 13. *******************************************************************************

// // Fill the blanks to make it work
// fn main() {
//     assert_eq!(i8::MAX, __);
//     assert_eq!(u8::MAX, __);

//     println!("Success!");
// }

fn ex13() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
}

// Ex 14. *******************************************************************************

// Fix errors and panics to make it work
// fn main() {
//     let v1 = 251_u8 + 8;
//     let v2 = i8::checked_add(251, 8).unwrap();
//     println!("{},{}",v1,v2);
//  }

fn ex14() {
    let v1: u16 = 251_u16 + 8;
    let v2: i16 = i16::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);
}

// Ex 15. *******************************************************************************

// // Modify `assert!` to make it work
// fn main() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//     assert!(v == 1579);

//     println!("Success!");
// }

fn ex15() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; //  1024 + 255 + 63 + 255
                                               // assert!(v != 1579)
    assert!(v == 1597);
}

// Ex 16. *******************************************************************************

// // Fill the blank to make it work
// fn main() {
//     let x = 1_000.000_1; // ?
//     let y: f32 = 0.12; // f32
//     let z = 0.01_f64; // f64

//     assert_eq!(type_of(&x), "__".to_string());
//     println!("Success!");
// }

// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

fn ex16() {
    let x: f64 = 1_000.000_1; // ?
    let _y: f32 = 0.12; // f32
    let _z: f64 = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
}

// Ex 17. *******************************************************************************

// fn main() {
//     assert!(0.1+0.2==0.3);

//     println!("Success!");
// }

fn ex17() {
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
}

// Ex 18. *******************************************************************************

// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i
//     }

//     assert!(sum == -3);

//     for c in 'a'..='z' {
//         println!("{}",c);
//     }

fn ex18() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}", c);
    }
}
