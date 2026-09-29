fn main() {
    second_function(5, 'f', false, 5.33, "Hello, world!");
    statement();
    let x = five();
    println!("The value of x is: {x}");
    let x = plus_one(66);
    println!("The value of x is: {x}");
}

fn second_function(x_int: i32, y_char: char, z_bool: bool, f_float:f32, s_string: &str) {
    println!("The value of x, y, z, f is: {x_int}, {y_char}, {z_bool}, {f_float}, {s_string}");
}
fn statement(){
    let x = 10;
    let y = 10;
    println!("The value of x and y is: {x}, {y}");
}

fn five() -> i32{
    5
}

fn plus_one(x:i64)->i64{
    x + 1
}