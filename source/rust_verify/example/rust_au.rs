use builtin::*;
use builtin_macros::*;
use vstd::{prelude::*, vec::*, *, ptr::*, string::*};

use vstd::option::*;

verus! {


fn binary_search(v: &Vec<i64>, k: i64) -> (index: usize)
    requires
        forall|i:int, j:int| 0 <= i <= j <= v.len() ==> v[i] <= v[j],
        exists|i:int| 0 <= i < v.len() && k == v[i],
    ensures
        index < v.len() && v[index as int] == k,
{
    let mut low = 0;
    let mut high = v.len()-1;

    while low != high 
        invariant
            high < v.len(), 
            exists|i:int| low <= i <= high && k == v[i], 
            forall|i:int, j:int| 0 <= i <= j < v.len() ==> v[i] <= v[j],
    {    
    }
    low
}


fn main() {}

} // verus! 
  




























// MIND BLANK HELP:
    /* let mut low = 0;
    let mut high = v.len() - 1;
    while low != high
        invariant
            high < v.len(),
            exists|i:int| low <= i <= high && k == v[i], 
            forall|i:int, j:int| 0 <= i < j < v.len() ==> v[i] <= v[j],
    {
        let mid = low + (high-low)/2;
        if *v.index(mid) < k {
            low = mid + 1;
        }else {
            high = mid;
        }
    }
    low */
