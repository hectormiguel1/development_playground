mod date;
mod student;

fn main() {
    let tmpDate = date::Date::new_from_string("07/15/1997");
    let tmp_student = student::Student::new_from("Hector", "Ramirez", tmpDate);
    println!("{}", tmp_student.to_string())
}
