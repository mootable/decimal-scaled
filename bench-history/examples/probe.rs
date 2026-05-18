use decimal_scaled::D307;
fn main() {
    let a = "1234.5".parse::<D307<50>>();
    println!("1234.5 @ SCALE=50: {:?}", a.err());
    let b = "1.5".parse::<D307<50>>();
    println!("1.5 @ SCALE=50: {:?}", b.err());
    let c = "1234.5".parse::<D307<10>>();
    println!("1234.5 @ SCALE=10: {:?}", c.err());
    let d = "12.34".parse::<D307<50>>();
    println!("12.34 @ SCALE=50: {:?}", d.err());
}
