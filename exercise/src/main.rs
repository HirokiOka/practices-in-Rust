fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i16 = 10;
    let y: i16 = 15;

    println!("{x} * {y} = {}", multiply(i16::from(x), y));
}
