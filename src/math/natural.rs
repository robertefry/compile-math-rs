
use super::*;

// Define zero as the empty set.
pub type Zero = EmptySet;

// Addition (Peano's axioms)

impl<T> AddOp<T> for Zero
{
    type Type = T;                  // 0 + T = T
}

impl<A: AddOp<B>, B> AddOp<B> for Succ<A>
{
    type Type = Succ<Add<A,B>>;     // S(A) + B = S(A+B)
}

// Multiplication (Peano's axioms)

impl<T> MulOp<T> for Zero
{
    type Type = Zero;               // 0 * T = 0
}

impl<A: MulOp<B>, B> MulOp<B> for Succ<A>
where
    Mul<A,B>: AddOp<B>
{
    type Type = Add<Mul<A,B>,B>;    // S(A) * B = B + A*B
}


// Format nicely readable for mere humans.
pub trait AsNatural
{
    fn as_natural() -> usize;
}

impl AsNatural for Zero
{
    fn as_natural() -> usize
    {
        0
    }
}

impl<T: AsNatural> AsNatural for Succ<T>
{
    fn as_natural() -> usize
    {
        1 + T::as_natural()
    }
}


// Tests

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn zero()
    {
        assert_eq!(Zero::as_natural(), 0);
        assert_eq!(Zero::as_set(), "{}");
    }

    #[test]
    fn successors()
    {
        type One = Succ<Zero>;
        assert_eq!(One::as_natural(), 1);
        assert_eq!(One::as_set(), "{{}}");

        type Two = Succ<One>;
        assert_eq!(Two::as_natural(), 2);
        assert_eq!(Two::as_set(), "{{{}}}");
    }
}
