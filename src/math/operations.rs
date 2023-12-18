
// Addition

pub trait AddOp<T>
{
    type Type;
}

pub type Add<A,B> = <A as AddOp<B>>::Type;

// Multiplication

pub trait MulOp<T>
{
    type Type;
}

pub type Mul<A,B> = <A as MulOp<B>>::Type;
