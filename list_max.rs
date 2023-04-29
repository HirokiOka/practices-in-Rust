fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("{}", &get_largest_i32(&number_list));

    let char_list = vec!['a', 'l', 'i', 'c', 'e'];
    println!("{}", &get_largest_char(&char_list));
}

fn get_largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn get_largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
