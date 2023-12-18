
// The empty set.
pub struct EmptySet;

// The successor function.
pub struct Succ<T> (T);


// Format nicely readable for mere humans.
pub trait AsSet
{
    fn as_set() -> String;
}

impl AsSet for EmptySet
{
    fn as_set() -> String
    {
        format!("{{}}")
    }
}

impl<T: AsSet> AsSet for Succ<T>
{
    fn as_set() -> String
    {
        format!("{{{}}}", T::as_set())
    }
}


// Tests

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn empty_set()
    {
        assert_eq!(EmptySet::as_set(), "{}");
    }

    #[test]
    fn successor_function()
    {
        assert_eq!(Succ::<EmptySet>::as_set(), "{{}}");
    }
}
