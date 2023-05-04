use magnus::{embed, eval};

fn main() {
    println!("Hello, world!");
    
    let _cleanup = unsafe { embed::init() };

    let val: String = eval!("RUBY_VERSION").unwrap();

    println!("{}", val);
}
