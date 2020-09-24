pub struct Date {
    month: i8,
    day: i8,
    year: i32
}
impl Date {
    pub fn new() -> Date {
        Date{
            month: 0,
            day: 0,
            year: 0
        }
    } 

    pub fn new_at_date(month: i8, day: i8, year :i32) -> Date {
        Date{month: month, day: day, year: year}     
    }

    pub fn to_string(&self) -> String {
        format!{"{}//{}//{}",self.month, self.day, self.year}
    }
    pub fn change_date(&mut self, new_month: i8, new_day: i8, new_year: i32) 
    {
        self.month = new_month;
        self.day = new_day;
        self.year = new_year;
    }

    pub fn change_date_to_date(&mut self, new_date: Date) {
        self.month = new_date.month;
        self.day = new_date.day;
        self.year = new_date.year;
    }
}
