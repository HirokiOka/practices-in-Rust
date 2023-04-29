fn get_largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if largest < item {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![12, 4, 6, 2];
    let result = get_largest(&number_list);
    println!("{}", result);
}
