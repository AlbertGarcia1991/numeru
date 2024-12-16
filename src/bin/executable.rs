extern crate numeru;

use numeru::data_structures::ds_array::array_static::ArrayStatic;

fn main() {
    let a: ArrayStatic = ArrayStatic {capacity: 10};
    println!("{}", a);
}