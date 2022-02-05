use std::marker::PhantomData;

trait Nat {
    type Pred: Nat;
}
struct Zero;
impl Nat for Zero {
    type Pred = Zero;
}
struct Succ<T: Nat>(PhantomData<T>);
impl<T: Nat> Nat for Succ<T> {
    type Pred = T;
}

trait IsTrue {}

struct Equal<A, B>(PhantomData<(A, B)>);
impl<N, A: Val<V = N>, B: Val<V = N>> IsTrue for Equal<A, B> {}

struct GreaterThanOrEqual<A: Nat, B: Nat>(PhantomData<(A, B)>);
impl<A: Nat> IsTrue for GreaterThanOrEqual<A, Zero> {}
impl<A: Nat, B: Nat> IsTrue for GreaterThanOrEqual<Succ<A>, Succ<B>>
    where GreaterThanOrEqual<A, B>: IsTrue
{}

trait Val {
    type V: Nat;
}
impl<N: Nat> Val for N {
    type V = N;
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


struct Sub<A: Nat, B: Nat>(PhantomData<(A, B)>);
// A - 0 = 0
impl<A: Nat> Val for Sub<A, Zero> {
    type V = A;
}
// A - B = C => A + Succ<B> = C::Pred
impl<A: Nat, B: Nat, C: Nat> Val for Sub<A, Succ<B>>
    where Sub<A, B>: Val<V = C>
{
    type V = C::Pred;
}

fn assert<T: IsTrue>() {}

fn main() {
    assert::<GreaterThanOrEqual<Succ<Zero>, Zero>>();
    assert::<GreaterThanOrEqual<Zero, Zero>>();
    assert::<Equal<Sub< Succ<Succ<Succ<Zero>>>, Succ<Succ<Zero>> >, Succ<Zero>>>();
    assert::<GreaterThanOrEqual<Zero, Succ<Zero>>>();
}
