// use bevy::prelude::*;

// fn main() {
//     App::build()
//         .add_startup_system(add_people.system())
//         .add_system(hello_world.system())
//         .run();
// }

// fn hello_world() {
//     let mut n = 0;
//     // Addition
//     println!("1 + 2 = {}", 1u32 + 2);

//     // Subtraction
//     println!("1 - 2 = {}", 1i32 - 2);
//     // ^ Try changing `1i32` to `1u32` to see why the type is important

//     // Integer Division
//     println!("9 / 2 = {}", 9u32 / 2);

//     // Float Division
//     println!("9 / 2 = {}", 9.0 / 2.0);

//     // Multiplication
//     println!("3 * 6 = {}", 3 * 6);
//     println!("hello world! {}", n);
// }

// struct Person;

// struct Name(String);

// fn add_people(mut commands: Commands) {
//     commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
//     commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
//     commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
// }
