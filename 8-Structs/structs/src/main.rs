#[derive(Debug)]
#[allow(dead_code)]
struct Student {
    name: String,
    roll_number: String,
    email: String,
    is_active: bool,
    group_number: u8,
    semester: u8,
    reference: String
}

fn main() {
    /*
    * Normal instantiation
    */
    let mut student = Student {
        name: String::from("Aryan"),
        roll_number: String::from("1234567890"),
        email: String::from("aryan@college_email.com"),
        is_active: true,
        group_number: 11,
        semester: 8,
        reference: String::from("Random")
    };
    
    student.name = String::from("Aryan Chopra");
    
    println!("{:#?}", student);
    
    let student_2 = build_student(String::from("Arjun"), String::from("1234567880"), String::from("arjun@college_email.com"), 11, 8);
    
    println!("{:#?}", student_2);
    
    /*
    * Struct update syntax
    * `student` object is still valid because only the values on the stack have been COPIED
    */
    let student_3 = Student{
        name: String::from("Arnav"),
        roll_number: String::from("1234567870"),
        email: String::from("arnav@college_email.com"),
        reference: String::from("Random"),
        ..student
    };
    
    println!("{:#?}", student_3);
    
    /*
    * `student_2` object is now invalid because:
    * `reference` is a String object stored on heap and it has been MOVED from student_2 to student_4 now
    * We can still use other fields other than the moved one
    */
    let student_4 = Student {
        name: String::from("Aman"),
        roll_number: String::from("12345667890"),
        email: String::from("aman@college_email.com"),
        ..student_2
    };
    
    /*
    * use of moved value: student_2.reference
    move occurs because student_2.reference has type String, which does not implement the Copy trait (rustc E0382)
    hint: value moved here
    */
    // let reference = student_2.reference;

    println!("{:#?}", student_4);
}


/**
* Field init shorthand syntax
*/
fn build_student(name: String, roll_number: String, email: String, group_number: u8, semester: u8) -> Student {
    Student {
        name, 
        roll_number, 
        email, 
        is_active: true,
        group_number,
        semester,
        reference: String::from("Sample")
    }
}
