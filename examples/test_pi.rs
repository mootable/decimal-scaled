use decimal_scaled::{D57, D115, D462, D616, D924, D1232, DecimalConstants};
fn main() {
    println!("D57<5> pi  = {:?}", D57::<5>::pi());
    println!("D115<10> pi = {:?}", D115::<10>::pi());
    println!("D462<10> pi = {:?}", D462::<10>::pi());
    println!("D616<10> pi = {:?}", D616::<10>::pi());
    println!("D924<10> pi = {:?}", D924::<10>::pi());
    println!("D1232<10> pi = {:?}", D1232::<10>::pi());
}
