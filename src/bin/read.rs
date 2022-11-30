use std::env;

use plum::{error::ChumskyAriadne, interpreter::interpret_full};

fn main() {
    let path = &env::args().collect::<Vec<String>>()[1];
    let file = std::fs::read(path).unwrap();
    let source = String::from_utf8(file).unwrap();
    let source = source.as_ref();

    let evaluated = interpret_full(source);

    match evaluated {
        Err(errs) => {
            for err in errs {
                err.display(path, source, 0);
            }
        }
        Ok(out) => println!("{:#?}", out),
    }
}
