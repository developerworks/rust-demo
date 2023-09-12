#[allow(unused)]
#[derive(Debug, Default)]
struct Object {
    id: String,
    ephemeral: bool,
}

fn main() {
    let o = Object::default();

    println!("o: {:?}", o);
}