use vstd::{*, ptr::*, vec::*, slice::*};
use builtin::*;
use builtin_macros::*;

// [0. 1]
// we are looking for 0
// low = 0
// high = 1
// mid = 0
// c1 -> false
// c2 -> true
// high = 0
// exit
//
//
// [0. 1]
// we are looking for 1
// low = 0
// high = 1
// mid = 0
// c1 -> true
// c2 -> false
// high = 0
// exit
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
        if *v.index(mid) < k {
            low = mid + 1;
        }else {
            high = mid;
        }
    }
    low
}







/* fn sum(vs: &[u32]) -> u64 
{
    let mut accum: u64 = 0;
    let mut i = 0; 
    while i < slice_len(vs)
        invariant 
            i < vs@.len()
    {
        accum = accum + (*slice_index_get(vs, i) as u64);
        i = i + 1;
    }
    accum
} */












// MIND BLANK HELP:
//
    /* let mut low = 0;
    let mut high = v.len() - 1;
    while low != high {
        let mid = (high + low) / 2;
        if v[mid] < k {
            // c1
            low = mid + 1;
        } else {
            //c2
            high = mid;
        }
    }
    low */



fn typically_unsafe() {
    let (iptr, Tracked(perm)) = PPtr::<u32>::new(45); 

    let tracked mut perm: PointsTo<u32> = perm;

    let integer: usize = iptr.to_usize();
    let iptr2: PPtr<u32> = PPtr::from_usize(integer);

    let val = iptr2.take(Tracked(&mut perm));
    assert(val == 45);
}

}
fn main() {}


























