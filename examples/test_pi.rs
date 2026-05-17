use decimal_scaled::{D56, D114, D461, D615, D923, D1231, DecimalConsts};
fn main() {
    println!("D56<5> pi  = {:?}", D56::<5>::pi());
    println!("D114<10> pi = {:?}", D114::<10>::pi());
    println!("D461<10> pi = {:?}", D461::<10>::pi());
    println!("D615<10> pi = {:?}", D615::<10>::pi());
    println!("D923<10> pi = {:?}", D923::<10>::pi());
    println!("D1231<10> pi = {:?}", D1231::<10>::pi());
}
