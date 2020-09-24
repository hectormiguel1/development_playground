pub struct Date {
    month: i8,
    day: i8,
    year: i32
}
impl Date {
    //Creates a new date object with the 00/00/000 default date 
    pub fn new() -> Date {
        Date{
            month: 0,
            day: 0,
            year: 0
        }
    } 
    //Creates a date with a starting values.
    pub fn new_at_date(month: i8, day: i8, year :i32) -> Date {
        Date{month: month, day: day, year: year}     
    }
    
    //Creates a Date object from a string
    pub fn new_from_string(date_string: &str) -> Date {
        let split_string: Vec<&str> = date_string.split("/").collect();
        if split_string.len() == 3 {
            return Date {
                month: String::from(split_string[0]).parse::<i8>().unwrap(),
                day: String::from(split_string[1]).parse::<i8>().unwrap(),
                year: String::from(split_string[2]).parse::<i32>().unwrap(),
            }
        }
        else {return Date::new()}
    }

    /*
    Returns the date in the MM/DD/YYYY Format
    */
    pub fn to_string(&self) -> String {
        format!{"{:02}/{:02}/{:04}",self.month, self.day, self.year}
    }
    /*
    Returns the date in the DD/MM//YYYY format
    */
    pub fn to_string_international(&self) -> String {
        format!{"{:02}/{:02}/{:04}", self.day, self.month, self.year}
    }

    //Changes the date to a new date, by passing in new month, new day and new year
    pub fn change_date(&mut self, new_month: i8, new_day: i8, new_year: i32) 
    {
        self.month = new_month;
        self.day = new_day;
        self.year = new_year;
    }
    //Change date to a new date object (Clones that date)
    pub fn change_date_to_date(&mut self, new_date: Date) {
        self.month = new_date.month;
        self.day = new_date.day;
        self.year = new_date.year;
    }
}
