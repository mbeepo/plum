use std::env;

use plum::eval::eval;

fn main() {
    let path = &env::args().collect::<Vec<String>>()[1];
    let file = std::fs::read(path).unwrap();
    let evaluated = eval(String::from_utf8(file).unwrap());

    println!("{:#?}", evaluated);
}
