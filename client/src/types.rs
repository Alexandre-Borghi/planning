use std::collections::HashMap;

use chrono::NaiveDate;

pub type Days = HashMap<NaiveDate, String>;
pub type Timeslots = HashMap<String, String>;
