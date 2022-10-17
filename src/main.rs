mod addition;
mod submodules;

fn main() {
    println!("{:?}", addition::add(5,10));
    println!("{:?}", submodules::subtraction::sub(5,10));
}
