/*
 * Why can't a type implement both Copy and Drop
 */

use std::alloc::{alloc, dealloc, Layout};

#[derive(Copy, Clone)]
struct Student {
    id: *mut u32
}

fn main() {
    unsafe {
        let layout = Layout::new::<u32>();
        let ptr = alloc(layout);
        
        *(ptr as *mut u32) = 1;
        
        let s2: Student;
        
        {
            let s1 = Student{
                id: ptr as *mut u32
            };
            
            s2 = s1;
            
            println!("{:?}", *s1.id);
            println!("{:?}", *s2.id);
            
            // Emulating `drop` call when scope ends
            dealloc(s1.id as *mut u8, layout);
        }
        
        // Shows garbage
        println!("{:?}", *s2.id);
    }
}
