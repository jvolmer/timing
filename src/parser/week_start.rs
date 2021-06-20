use chrono::prelude::*;
use std::error::Error;

#[derive(Debug, PartialEq)]
struct TimingDate {
    date: Date<Local>
}

impl TimingDate {
    fn parse(string: &str) -> Result<Self, Box<dyn Error>> {
	let naive_date = NaiveDate::parse_from_str(string, "* %Y-%m-%d")?;
	Ok(
	    TimingDate {
		date: Local.from_local_date(&naive_date).unwrap()
	    }
	)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_week_line() {
	let line = "* 2021-05-10";

	let week_start = TimingDate::parse(line).unwrap();

	assert_eq!(week_start, TimingDate { date: Local.ymd(2021, 5, 10) });
    }
}
