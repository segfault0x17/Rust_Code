const TWO: u32 = 1 + 1;

fn func4(){
    let x = 15;
    let x = "16";
    println!("The value of x is: {x}");
}

fn func3() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("func 3: The value of x in the inner scope is: {x}");
    }
    println!("func 3: The value out of the inner scope is: {x} ");
}
fn func2(){
    println!("{TWO}");
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    func2();
    func3();
    func4();
}
