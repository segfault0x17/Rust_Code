fn compound_types_array(){
    let a: [i64; 5] = [1, 2, 3, 4, 5];
    println!("{a:?}");

    let first = a[0]; // first element of the array
    let second = a[1]; // second element of the array
    println!("{first}\n{second}");

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("{months:?}");
    let fifth_month = months[4];
    println!("{fifth_month}");
}

fn compound_types_tuple(){
    let tup: (i32, f64, u8, i16) = (500, 5.5, 2, 1000);
    println!("{tup:?}");

    let (x, y, z, w) = tup; // destructuring
    println!("{y}");

    let five_hundred = tup.0;
    let five_point_five = tup.1;
    let two = tup.2;
    let one_thousand = tup.3 + 2; // addition 2 to the third value of the tuple
    println!("{five_hundred}\n{five_point_five}\n{two}\n{one_thousand}");
}

fn scalar_types(){
    let guess: u32 = "42".parse().expect("Not a number!");
    let guess1 = "42";
    println!("{guess}");
    println!("{guess1}");

    let x = 2.0;
    let y: f32 = 4.0;
    println!("{x}\n{y}");

    let t: bool = true;

    let f: bool = false;

    println!("{t}\n{f}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{c}\n{z}\n{heart_eyed_cat}");
}


fn main() {
    println!("compound type tuple output function");
    compound_types_tuple();
    println!("================================");
    println!("compound type array output function");
    compound_types_array();
    println!("================================");
    println!("scalar type output function");
    scalar_types();
}
