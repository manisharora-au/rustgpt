mod m1_enums;
mod m1_structs;
mod mi_traits;
mod my_polymorph;
mod m1_lifetimes;

const COURSE: &str = "Rust with AutoGPT";

fn main() {
    println!("Welcome to this course on {}", COURSE);
    // arrays
    let arr = [1.0; 10];
    // Create a new array from arr
    let mut new_arr = arr.map(|n| n + 3.8);
    println!("new_arr 1 {:?} ", new_arr);
    // Update each element within new_arr with adding 50.5
    new_arr.iter_mut().for_each(|n| {
        *n += 50.5;
    });
    println!("new_arr 2 {:?} ", new_arr);
    // Vectors
    let vec1 = vec![3; 10];
    let mut vec2: Vec<_> = vec1.iter().map(|n| n * 2).collect();
    println!("vec2 1 {:?}", vec2);
    // OR do this
    let random_number = 3;
    vec2.iter_mut().for_each(|x| {
        *x /= 5 + random_number;
    });
    println!("vec2 2 {:?}", vec2);

    // Strings and chars
    //let name = String::from("manish arora");
    // OR
    let name: &str = "manish arora";
    let mut char_vec: Vec<char> = name.chars().collect();
    char_vec.iter_mut().for_each(|c| {
        *c = char::to_ascii_uppercase(c);
    });
    println!("New char_vec {:?}", &char_vec);

    // Closures
    let some_number = 50;
    let add_to_some_number = |x: i32| x + some_number;
    let result = add_to_some_number(4);
    dbg!("add_to_some_number {}", &result);

    // Raw - String Litrerals
    let text = r#"RUST is just "awesome" "#;
    dbg!(&text);
}
