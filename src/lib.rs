use std::marker::PhantomData;

trait Nat {}
struct Zero;
impl Nat for Zero {}
struct Succ<T: Nat>(PhantomData<T>);
impl<T: Nat> Nat for Succ<T> {}

trait IsTrue {}

struct GreaterThanOrEqual<A: Nat, B: Nat>(PhantomData<(A, B)>);
impl<A: Nat> IsTrue for GreaterThanOrEqual<A, Zero> {}
impl<A: Nat, B: Nat> IsTrue for GreaterThanOrEqual<Succ<A>, Succ<B>>
    where GreaterThanOrEqual<A, B>: IsTrue
{}

trait Val {
    type V: Nat;
}

struct Add<A: Nat, B: Nat>(PhantomData<(A, B)>);
// A + 0 = A
impl<A: Nat> Val for Add<A, Zero> {
    type V = A;
}

// A + B = C => A + Succ<B> = Succ<C>
impl<A: Nat, B: Nat, C: Nat> Val for Add<A, Succ<B>>
    where Add<A, B>: Val<V = C>
{
    type V = Succ<C>;
}

fn assert<T: IsTrue>() {}

fn main() {
    assert::<GreaterThanOrEqual<Succ<Zero>, Zero>>();
    assert::<GreaterThanOrEqual<Zero, Zero>>();
    assert::<GreaterThanOrEqual<Zero, Succ<Zero>>>();
}
