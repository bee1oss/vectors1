fn main() {
    let mut list = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}",addelement(&mut list));

}

fn addelement(list: &mut Vec<i32>) -> &mut Vec<i32> {
    list.push(1);
    list.insert(2, 0);
    return list;
}
