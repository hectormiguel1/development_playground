use crate::date::Date;
use uuid::Uuid;

pub struct Student {
    f_name: String,
    l_name: String,
    dob: Date,
    ID: Uuid
}

impl Student {
    pub fn new() -> Student  {
        Student{
            f_name: String::from(""),
            l_name: String::from(""),
            dob: Date::new(),
            ID: Uuid::new_v4()
        }
    }
    pub fn new_from(f_name: &str, l_name: &str, dob: Date) -> Student  {
        Student {
            f_name: f_name.to_string(),
            l_name: l_name.to_string(),
            dob: dob,
            ID: Uuid::new_v4()
        }
    }
    pub fn to_string(&self) -> String { 
        format!{"{} {} \nDOB:{}\nID: {}", self.f_name, self.l_name, self.dob.to_string(), self.ID}
    }
}