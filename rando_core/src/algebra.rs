
/// Truthy forms a lattice
pub trait Truthy {
    fn top() -> Self;
    fn bottom() -> Self;
    fn join(&self, other: &Self) -> Self;
    fn meet(&self, other: &Self) -> Self;
}

pub trait Sphery: Truthy {
    fn increment(&self) -> Self;
}

pub trait County<T: Truthy>: Truthy {
    /// add is an abelian monoid with bottom() as zero
    fn add(&self, other: &Self) -> Self;
    /// scale(add(x,y),n) = add(scale(x,n),scale(y,n))
    /// scale(scale(x,n),m) = scale(x, n*m)
    fn scale(&self, n: u32) -> Self;
    /// lift(bottom()) = bottom()
    /// lift(top()) = top()
    /// lift(join(x,y)) = join(lift(x), lift(y))
    /// lift(meet(x,y)) = meet(lift(x), lift(y))
    fn lift(truthy: &T) -> Self;
    /// ge(x,0) = Top
    /// ge(lift(x), n) = x for n > 0
    /// Scaling should probably make sense
    fn ge(&self, n: u32) -> T;
}