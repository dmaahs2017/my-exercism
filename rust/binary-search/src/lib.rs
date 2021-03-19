use std::cmp::PartialOrd;

/// Binary search. Returns None if element is not found. Otherwise returns Some(index)
pub fn find<C, T>(array: C, key: T) -> Option<usize>
where
    C: AsRef<[T]>,
    T: PartialEq + PartialOrd,
{
    let array = array.as_ref();
    if array.is_empty() {
        return None;
    }

    let mut slice = array;
    while let Some(mid) = get_mid(slice) {
        let elem = &slice[mid];
        if *elem == key {
            // so this is a little goofy, but I wanted to solve this by changing the scope of the
            // slice rather than track the left and right indicies. The issue was that `mid` would
            // be a relative idx to the current slice, rather than the index to the input array.
            // Sice slices are contiguous memory I'm able to calculate the "absolute" position in
            // the array given my relative position and the current slice with a bit of pointer
            // math.
            //
            // With this method we need not concern ourselves with a state machine. As long as our
            // current slice is not empty keep searching.

            // calculate absolute such that:
            //      array[absolute] == slice[mid]
            let absolute = (slice.as_ptr() as usize - array.as_ptr() as usize) // number of bytes `slice` is from `array`
                / std::mem::size_of::<T>() // turn into the number of indicies the start of `slice` is from `array`
                + mid;
            return Some(absolute);
        }
        if key > *elem {
            slice = &slice[mid + 1..];
        } else {
            slice = &slice[..mid];
        }
    }
    None
}

/// Fails if array is empty
fn get_mid<T>(arr: &[T]) -> Option<usize> {
    Some((arr.len().checked_sub(1)?) / 2)
}
