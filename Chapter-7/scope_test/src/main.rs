fn main() {
    let sp:*const String;
    
    {
        let mut s = String::from("Hello");
        sp = &s;
        s.push_str(", world");
        
        unsafe {
            println!("Inside: {:?}", *sp);
        }
    }
    
    unsafe {
        println!("Outside: {:?}", *sp);
    }
}
