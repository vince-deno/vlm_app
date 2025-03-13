

pub trait VLMSpanCore<T>
where
    T: Ord + Copy // ✅ Require `T` to support subtraction
{
    fn new(lo: T, hi: T) -> Self;
    fn range(&self) -> std::ops::Range<T>;
    fn len(&self) -> T; // ✅ `len()` must support subtraction
    fn is_empty(&self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
    fn intersection(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
    fn union(&self, other: &Self) -> Self;
    fn split_at(&self, position: T) -> (Self, Self)
    where
        Self: Sized;
}


pub trait VLMSpanUtils<T>
where
    T: Ord + Copy + std::fmt::Debug,
{
    fn print(&self);
}
