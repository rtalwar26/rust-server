pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    ManHours(u32, u32),
}
impl TimeUnit {
    pub fn plural(&self) -> String {
        match self {
            TimeUnit::Seconds => "seconds".to_string(),
            TimeUnit::Hours => "hours".to_string(),
            TimeUnit::Minutes => "minutes".to_string(),
            TimeUnit::ManHours(men, hours) => format!("{} hours", men * hours),
        }
    }
}
