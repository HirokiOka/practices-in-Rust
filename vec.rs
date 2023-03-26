fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let itr = nums
        .iter()
        .position(|&x, &y| x + y == target)
        .collect();
    print!("{}", itr);
}
