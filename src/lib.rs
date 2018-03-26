#![feature(fn_traits)]
#![feature(box_syntax)]
#![allow(non_snake_case)]
#![feature(associated_type_defaults)]
#![feature(unboxed_closures)]
#![feature(non_ascii_idents)]
#![feature(universal_impl_trait)]
#![feature(generators, generator_trait, step_trait)]
#![feature(fnbox)]
#![feature(get_type_id)]
#![feature(generic_param_attrs)]
#![feature(dropck_eyepatch)]
#![feature(nested_impl_trait)]
#![type_length_limit="33554432"]
#![feature(specialization)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(conservative_impl_trait)]
#![allow(non_snake_case)]

#[macro_use] pub mod util;

pub mod observable;
//pub mod subject;
//pub mod behaviour_subject;
pub mod op;
pub mod subref;
pub mod scheduler;
mod fac;
//pub mod connectable_observable;

pub use fac::*;

#[cfg(test)]
mod test_fixture;

#[cfg(test)]
mod test
{

}