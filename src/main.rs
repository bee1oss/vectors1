fn main() {
    let mut list = vec![1, 2, 3, 4, 5, 6];
    let check = gettingfirstelement(list.clone());

    if check == true {
        println!("{:?}", addelementpush(&mut list, 3));
        println!("{:?}", addelementinsert(&mut list, 3, 0));
        getelement(&mut list, 3);
        vector_capacity();
    } else {
        println!("If u're here it means something went wrong");
    }
}
fn vector_capacity(){
    let mut list1:Vec<i32> = Vec::with_capacity(4);
    println!("{}",list1.capacity());
    list1.push(1);
    list1.push(2);
    list1.push(3);
    list1.push(4);
    list1.push(5);
    list1.push(7);
    println!("{}",list1.capacity());
}

fn gettingfirstelement(list: Vec<i32>) -> bool {//can write better function with true false but i want it do latter
    match list.first() {
        Some(el) => {
            println!("First element is {}", el);
            return true;
        }
        None => return false
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
