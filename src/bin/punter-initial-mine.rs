extern crate punter;

fn main() {
    punter::main_helper(punter::Optimizer::InitialMine(punter::StateRater::Score));
}
