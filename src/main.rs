mod addition;
mod submodules;

fn main() {

    use addition::add as adder_function;
    use submodules::subtraction::sub as my_sub;

    println!("{:?}", adder_function(5,10));
    println!("{:?}", my_sub(5,10));
}
