use std::cmp::Ordering;
fn merge<T: Ord>(mut list_1: Vec<T>, mut list_2: Vec<T>) -> Vec<T>{
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
fn merge_sort<T: Ord>(mut merge_list: Vec<T>) -> Vec<T>{
    if merge_list.len() == 1 {return merge_list;}
    let mut recurse_list = merge_list.split_off(merge_list.len()/2);
    merge(merge_sort(merge_list), merge_sort(recurse_list))
}