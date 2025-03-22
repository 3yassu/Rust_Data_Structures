fn partition<T: Ord>(part_list: &mut Vec<T>, low: usize, high: usize) -> usize{
    let mut left = low;
    for i in low..high {
        if part_list[i] <= part_list[high]{
            part_list.swap(left, i);
            left += 1;
        }
    }
    part_list.swap(left, high);
    return left;
}
fn quick_sort<T: Ord>(quick_list: &mut Vec<T>, low: usize, high: usize){
    if low < high {
        let part = partition(quick_list, low, high);
        if part > 0 {quick_sort(quick_list, low, part - 1);}
        quick_sort(quick_list, part + 1, high);
    }
}