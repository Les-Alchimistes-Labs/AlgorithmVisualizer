pub fn insertion_sort(mut to_sort : &mut Vec<i64>)
{
	let len = to_sort.len();
    
    for i in 1..len {
        let mut j = i;
        while j > 0 && to_sort[j - 1] > to_sort[j] {
            to_sort.swap(j, j - 1);
            j -= 1;
        }
    }
}
