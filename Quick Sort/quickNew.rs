fn partition<T: Ord>(part_list: &mut Vec<T>) -> usize{
    let mut left = 0; let high = part_list.len() - 1;
    for i in 0..high {
        if part_list[i] <= part_list[high]{
            part_list.swap(left, i);
            left += 1;
        }
    }
    part_list.swap(left, high);
    return left;
}
fn quick_sort<T: Ord>(mut quick_list: Vec<T>) -> Vec<T>{
    if quick_list.len() <= 1 {return quick_list;}
    let part = partition(&mut quick_list); let mut recurse_item = quick_list.split_off(part);
    let mut left = quick_sort(quick_list); let mut right = quick_sort(recurse_item.split_off(1));
    left.append(&mut recurse_item); left.append(&mut right);
    return left;
}