#![feature(catch_expr)]

fn value() -> Result<i32, String> {
    Ok(12)
}

fn main() {
    let x = do catch {
        //value()
        12
    };

    x = ();
}