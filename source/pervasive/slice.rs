#![allow(unused_imports)]
use builtin::*;
use builtin_macros::*;
use crate::seq::*;
use crate::vec::*;

verus!{

pub trait SliceAdditionalSpecFns<T> {
   spec fn view(&self) -> Seq<T>;
   spec fn spec_index(&self, i: int) -> T;
}

impl<T> SliceAdditionalSpecFns<T> for [T] {
    spec fn view(&self) -> Seq<T>;

    #[verifier(inline)]
    open spec fn spec_index(&self, i: int) -> T {
        self.view().index(i)
    }
}

#[verifier(external_body)]
pub exec fn slice_index_get<T>(slice: &[T], i: usize) -> (out: &T)
    requires 0 <= i < slice.view().len(),
    ensures *out == slice@.index(i as int),
{
    &slice[i]
}

#[verifier(external_body)]
pub exec fn slice_to_vec<T: Copy>(slice: &[T]) -> (out: Vec<T>)
    ensures out@ == slice@
{
    Vec { vec: slice.to_vec() }
}

#[verifier(external_body)]
pub exec fn slice_subrange<T, 'a>(slice: &'a [T], i: usize, j: usize) -> (out: &'a [T])
    requires 0 <= i <= j <= slice@.len()
    ensures out@ == slice@.subrange(i as int, j as int)
{
    &slice[i .. j]
}

#[verifier(external_body)]
pub exec fn slice_len<T>(slice: &[T]) -> (r: usize)
    ensures r == slice.view().len(),
{
    slice.len() 
}

#[verifier(external_body)]
pub exec fn slice_get_unchecked<T, 'a>(slice: &'a [T], i: usize) -> (r: &'a T)
    requires 0 <= i < slice@.len()
    ensures r == slice@.index(i as int)
{
    unsafe { slice.get_unchecked(i) }
}


}
