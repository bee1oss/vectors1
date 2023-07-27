fn main() {
    let mut list = vec![1, 2, 3, 4, 5, 6];
    gettingfirstelement(list);
    println!("{:?}", addelementpush(&mut list, 3));
    println!("{:?}", addelementinsert(&mut list, 3, 0));
    getelement(&mut list, 3);

}

fn gettingfirstelement(list: Vec<i32>) {
    match list.first() {
        Some(el) => println!("First element is {}", el),
        None =>println!("Vector is empty!"),
    }
}

fn getelement(list: &mut Vec<i32>, index: usize) {
    match list.get(index) {
        Some(el) => println!("Element is {}", el),
        None => println!("Element with index {} doesnt exist!", index)
    }
}

fn addelementpush(list: &mut Vec<i32>, value: i32) -> &mut Vec<i32> {
    list.push(value);
    return list;
}

fn addelementinsert(list: &mut Vec<i32>, index: usize, value: i32) -> &mut Vec<i32> {
    list.insert(index, value);
    return list;
}
