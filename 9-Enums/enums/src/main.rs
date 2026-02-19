/**
 * Standard Enum
 */
#[allow(dead_code)]
#[derive(Debug)]
enum PrivilegeLevel {
    Admin,
    Peasant
}


/**
 * A struct utilizing a Enum and being part of another enum
 */
#[allow(dead_code)]
#[derive(Debug)]
struct BaseProgram {
    name: String,
    path: String,
    execution_level: PrivilegeLevel
}


/**
 * Enum with a type associated with it
 * It can also be associated with primitive types such as `u8`
 */
#[allow(dead_code)]
#[derive(Debug)]
enum Program {
    Java(BaseProgram),
    Rust(BaseProgram),
    C(BaseProgram)
}

/**
 * Somehow, Enums can have... methods? Anyhow, here's the example
 */
#[allow(dead_code)]
impl Program {
    fn execute(&self) {
        println!("Executing: {:#?}", self);
    }
}


/**
 * Enum with every member expecting a different set of values
 */
#[allow(dead_code)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}


/**
 * A struct utilizing an enum
 */
#[allow(dead_code)]
struct Process {
    name: String,
    privilege_level: PrivilegeLevel,
    id: u32,
    path: String
}

#[allow(unused_variables)]
fn main() {
    let process_privilege = PrivilegeLevel::Peasant;
    
    let process = Process {
        name: String::from("gnome"),
        privilege_level: PrivilegeLevel::Admin,
        id: 1,
        path: String::from("/usr/local/gnome")
    };
    
    let dbeaver = Program::Java(
        BaseProgram { 
            name: String::from("DBeaver"), 
            path: String::from("/usr/local/dbeaver.java"), 
            execution_level: PrivilegeLevel::Peasant }
    );
    
    let localhost = IpAddress::V4(127, 0, 0, 1);
    
    dbeaver.execute();
}
