use anyhow::Result;
use derive_more::{Add, Display, From, Into};

#[derive(PartialEq, Clone, Copy, From, Add, Into, Display)]
struct MyInt(i32);

#[derive(PartialEq, From, Into)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, From, Add, Display)]
enum MyEnum {
    #[display(fmt = "int: {}", _0)]
    Int(i32),
    Uint(u32),
    #[display(fmt = "nothing")]
    Nothing,
}

fn main() -> Result<()> {
    let my_int: MyInt = 10.into();
    let v = my_int + 20.into();
    println!("my_int: {}, v: {}", my_int, v);

    let v1: i32 = v.into();
    println!("v1: {}", v1);

    let e: MyEnum = 10i32.into();
    let e2: MyEnum = 20u32.into();
    let e3 = MyEnum::Nothing;
    println!("e: {}, e2: {}, e3: {}", e, e2, e3);
    println!("e: {:?}, e2: {:?}, e3: {:?}", e, e2, e3);

    Ok(())
}
