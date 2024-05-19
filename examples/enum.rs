use anyhow::Result;
use serde::{Deserialize, Serialize};
use strum::{
    Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, IntoEnumIterator,
    IntoStaticStr, VariantNames,
};

#[allow(unused)]
#[derive(Display, Debug, Serialize, Deserialize)]
enum Color {
    #[strum(serialize = "redred", to_string = "red")]
    Red,
    Green {
        range: usize,
    },
    Blue(usize),
    Yellow,
    #[strum(to_string = "purple with {sat} saturation")]
    Purple {
        sat: usize,
    },
}

#[allow(unused)]
#[derive(
    Debug, EnumString, EnumCount, EnumDiscriminants, EnumIter, EnumIs, IntoStaticStr, VariantNames,
)]
enum MyEnum {
    A,
    B(String),
    C,
}

fn main() -> Result<()> {
    println!("{:?}", MyEnum::VARIANTS);
    MyEnum::VARIANTS.iter().for_each(|v| println!("{:?}", v));
    MyEnum::iter().for_each(|v| println!("{:?}", v));
    println!("enum count: {:?}", MyEnum::COUNT);

    let my_enum = MyEnum::B("hello".to_string());
    println!("{:?}", my_enum.is_b());

    let s: &'static str = my_enum.into();
    println!("{:?}", s);

    let red = Color::Red;
    let green = Color::Green { range: 10 };
    let blue = Color::Blue(20);
    let yellow = Color::Yellow;
    let purple = Color::Purple { sat: 30 };
    println!("red: {}", red);
    println!("green: {}", green);
    println!("blue: {}", blue);
    println!("yellow: {}", yellow);
    println!("purple: {}", purple);

    let red_str = serde_json::to_string(&red)?;
    let green_str = serde_json::to_string(&green)?;
    let blue_str = serde_json::to_string(&blue)?;
    let yellow_str = serde_json::to_string(&yellow)?;
    let purple_str = serde_json::to_string(&purple)?;
    println!("red_str: {}", red_str);
    println!("green_str: {}", green_str);
    println!("blue_str: {}", blue_str);
    println!("yellow_str: {}", yellow_str);
    println!("purple_str: {}", purple_str);
    Ok(())
}
