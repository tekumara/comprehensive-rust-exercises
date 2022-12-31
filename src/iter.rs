fn it1(mut iter: std::slice::Iter<i8>) -> () {
    let i: Option<&i8> = iter.next();
    println!("v[0]: {:?}", i);
    println!("v[1]: {:?}", iter.next());
    println!("v[2]: {:?}", iter.next());
    println!("No more items: {:?}", iter.next());
}


fn it2(mut iter: std::vec::IntoIter<i8>) -> () {
    let i: Option<i8> = iter.next();
    println!("v[0]: {:?}", i);
    println!("v[1]: {:?}", iter.next());
    println!("v[2]: {:?}", iter.next());
    println!("No more items: {:?}", iter.next());
}

fn main() {
    let v1: Vec<i8> = vec![10, 20, 30];
    let v2: Vec<i8> = v1.clone();

    println!("begin: {v1:?}");
    println!("begin: {v2:?}");

    let iter1: std::slice::Iter<i8> = v1.iter();
    let iter2: std::slice::Iter<i8> = (&v2).into_iter();

    it1(iter1);

    println!("done: {v1:?}");

    //it2(iter2);

    for i in v2 {

    }

    println!("done: {v2:?}");

}
