mod addition;
mod submodules;

fn main() {

    use addition::add;
    use submodules::subtraction::sub;

    println!("{:?}", add(5,10));
    println!("{:?}", sub(5,10));
}
