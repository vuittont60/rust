#![feature(type_alias_impl_trait)]

fn main() {}

type Two<'a, 'b> = impl std::fmt::Debug;
//~^ ERROR unconstrained opaque type

fn one<'a>(t: &'a ()) -> Two<'a, 'a> {
    t
    //~^ ERROR non-defining opaque type use
}
