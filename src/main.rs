
mod math; use math::*;

fn main()
{
    type One    = Succ<Zero>;
    type Two    = Succ<One>;
    type Three  = Succ<Two>;

    type ThreePlusTwo = Add<Three,Two>;
    println!("3 + 2 = {}", ThreePlusTwo::as_set());
    println!("      = {}", ThreePlusTwo::as_natural());

    type ThreeTimesTwo = Mul<Three,Two>;
    println!("3 * 2 = {}", ThreeTimesTwo::as_set());
    println!("      = {}", ThreeTimesTwo::as_natural());
}
