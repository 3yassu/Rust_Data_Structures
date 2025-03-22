use std::cmp::Ordering;
fn merge<T: Ord>(merge_list: &mut Vec<T>, low: usize, middle: usize, high: usize) -> Vec<T>{
    let mut return_list = Vec::new();
    while (list_1.len() > 0) || (list_2.len() > 0) {
        if list_1.len() == 0 {return_list.push(list_2.remove(0));}
        else if list_2.len() == 0 {return_list.push(list_1.remove(0));}
        else{ 
            match list_1[0].cmp(&list_2[0]){
                Ordering::Less => return_list.push(list_1.remove(0)),
                Ordering::Equal => {return_list.push(list_1.remove(0)); return_list.push(list_2.remove(0));},
                Ordering::Greater => return_list.push(list_2.remove(0))
            }
        }
    }
    return_list
}
fn merge_sort<T: Ord>(merge_list: &mut Vec<T>, low: usize, high: usize){
    if low < high {
        middle = low+(high-low)/2;
        merge_sort(merge_list, low, middle); 
        merge_sort(merge_list, middle, high);
        merge(merge_list, low, middle, high);
    }
}