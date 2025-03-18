use std::cmp::Ordering;
fn partition<T: Ord>(&mut part_list: Vec<T>,> low: usize, high: usize) -> Vec<T>{
    let max = part_list.len() - 1;
    let pivot = part_list[max];
    let left = low - 1;
    for i in 0..max {
        if part_list[i] <= pivot{
            left += 1;
            let temp = part_list[i];
            part_list[i] = part_list[left];
            part_list[left] = temp;
        }{
            let temp = part_list[max];
            part_list[max] = part_list[left];
            part_list[left] = temp;
        }
        left + 1
    }
}
fn quick_sort<T: Ord>(&mut quick_list: Vec<T>, low: usize, high: usize){
    if low < high {
        part = partition(&mut quick_list, low, high);
        quick_sort(quick_list, low, part - 1); quick_sort(quick_list, part + 1, high);
    }
}