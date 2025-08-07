// struct Rect {
//     width: u32,
//     height: u32,
// }
// struct Square {
//     side: u32,
// }

// trait Shape {
//     fn area(&self) -> u32;
// }

// impl Shape for Rect {
//     fn area(&self) -> u32 {
//         return self.height * self.height;
//     }
// }

// impl Shape for Square {
//     fn area(&self) -> u32 {
//         return self.side * self.side;
//     }
// }

// #[derive(Debug)]
// struct S;
// fn main() {
//     let r = Rect {
//         width: 10,
//         height: 20,
//     };
//     let s = Square { side: 23 };
//     println!("the area of the Rectangle is {}", r.area());
//     println!("the area of the Squre is {}", s.area());
//     println!("the area of the Squre is {}", s.area());
//     println!("{:?}", S);
// }

struct Rect {
    height: u64,
    width: u64,
}

struct Squre {
    side: u64,
}

impl Rect {
    fn area(&self) -> u64 {
        return self.height * self.width;
    }
}

impl Squre {
    fn area(&self) -> u64 {
        return self.side * self.side;
    }
}

fn main() {
    let r = Rect {
        width: 10,
        height: 40,
    };

    let s = Squre { side: 39 };

    println!("the value of  reactangle {}", r.area());
    println!("the value of  squre {}", s.area());
}
