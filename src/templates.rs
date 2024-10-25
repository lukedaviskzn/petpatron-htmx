use crate::db;


pub fn fuzzy_time_delta(d: chrono::TimeDelta) -> String {
    let num_years = d.num_days() / 365;
    let num_months = d.num_days() / 30;
    let num_weeks = d.num_days() / 7;
    let num_days = d.num_days();
    let num_hours = d.num_hours();
    let num_minutes = d.num_minutes();
    let num_seconds = d.num_seconds();
    let num_milliseconds = d.num_milliseconds();

    fn fmt_unit(count: i64, single: &str, plural: &str) -> String {
        let unit = if count.abs() == 1 { single } else { plural };
        format!("{count} {unit}")
    }

    if num_years.abs() >= 1 {
        fmt_unit(num_years, "year", "years")
    } else if num_months.abs() >= 1 {
        fmt_unit(num_months, "month", "months")
    } else if num_weeks.abs() >= 1 {
        fmt_unit(num_weeks, "month", "months")
    } else if num_days.abs() >= 1 {
        fmt_unit(num_days, "day", "days")
    } else if num_hours.abs() >= 1 {
        fmt_unit(num_hours, "hour", "hours")
    } else if num_minutes.abs() >= 1 {
        fmt_unit(num_minutes, "minute", "minutes")
    } else if num_seconds.abs() >= 1 {
        fmt_unit(num_seconds, "second", "seconds")
    } else if num_milliseconds.abs() >= 1 {
        fmt_unit(num_milliseconds, "millisecond", "milliseconds")
    } else {
        "<1 millisecond".into()
    }
}

#[derive(rinja::Template)]
#[template(path = "index.html")]
pub struct Index {}

pub struct Thanks {
    pub dog_index: usize,
    pub dog_id: i64,
}

#[derive(rinja::Template)]
#[template(path = "dogs.html")]
pub struct Dogs {
    pub shelter: usize,
    pub dogs: Vec<db::Dog>,
    pub thanks: Option<Thanks>,
}

#[derive(rinja::Template)]
#[template(path = "dog_viewer.html")]
pub struct DogViewer {
    pub shelter: usize,
    pub dog: db::Dog,
    pub appreciative: bool,
}
