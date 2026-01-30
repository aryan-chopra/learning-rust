#[allow(dead_code)]
struct Student {
    name: String,
    roll_number: String,
    group_number: u16
}

impl Student {
    fn print_professors_of_student(&self, professors: &mut Vec<Professor>) {
        let student_name = &self.name;
        
        println!("Professors of {student_name}:");
        
        /*
         * Black magic... I guess?
         * Will update when I figure out why it doesn't work
         */
        for professor in professors.iter_mut() {
            for group_number in professor.teaches_groups.iter_mut() {
                if *group_number == self.group_number {
                    println!("{:#?}", &professor);
                    break;
                }
            }
        }
    }
    
    fn is_in_front_of(&self, other: &Student) -> bool {
        self.roll_number < other.roll_number
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Professor {
    name: String,
    email: String,
    teaches_groups: Vec<u16>
}

impl Professor {
    
    /**
     * Associated function
     */
    fn from(name: String, email: String) -> Professor {
        Professor { name, email, teaches_groups: Vec::new()}
    }
}

fn main() {
    let students = initialize_students();
    let mut professors = initialize_professors();
    
    /*
     * OH MY GOD I NEED TO FIGURE THIS OUT AS WELL T_T
     */
    for student in &students {
        student.print_professors_of_student(&mut professors);
    };
    
    let student_one_ahead_of_student_2 = students[0].is_in_front_of(&students[1]);
    
    println!("Is {} ahead of {} ? {}", students[0].name, students[1].name, student_one_ahead_of_student_2);
    
    /*
     * Using an associated function to create an instance of a struct
     */
    let new_professor = Professor::from(String::from("Someone sure"), String::from("random@email.com"));
    
    println!("New professor: {:#?}", new_professor);
}

fn initialize_students() -> Vec<Student> {
    let mut students = Vec::new();
    
    students.push(Student{
        name: String::from("Aryan"),
        roll_number: String::from("1234"),
        group_number: 1
    }
    );
    
    students.push(Student {
        name: String::from("Arjun"),
        roll_number: String::from("12345"),
        group_number: 2
    }
    );

    students.push(Student {
        name: String::from("Arnav"),
        roll_number: String::from("1243"),
        group_number: 3
    }
    );
    students
}

fn initialize_professors() -> Vec<Professor> {
    let mut professors = Vec::new();
    
    professors.push(Professor{
        email: String::from("a@b.com"),
        name: String::from("Random"),
        teaches_groups: vec![1, 3]
    }
    );
    
    professors.push(Professor{
        email: String::from("b@b.com"),
        name: String::from("Someone"),
        teaches_groups: vec![1, 2]
    }
    );
    
    professors.push(Professor{
        email: String::from("c@b.com"),
        name: String::from("Somebody"),
        teaches_groups: vec![2, 3]
    }
    );

    professors
}

