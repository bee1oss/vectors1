fn main() {
    let mut list = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}",addelementpush(&mut list,3));

}

fn addelementpush(list: &mut Vec<i32>,value:i32) -> &mut Vec<i32> {
    list.push(value);
    return list;
}
fn addelementinsert(list: &mut Vec<i32>,index:usize,value:i32) -> &mut Vec<i32> {
    list.insert(index, value);
    return list;
}
