fn main() {
    let ptr1 = "Hello";
    let ptr2 = "Hello";
    let ptr3 = "Hello";
    
    print_type(&ptr1);
    println!("{:p}", ptr1);
    println!("{:p}", ptr2);
    println!("{:p}", ptr3);
}

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
