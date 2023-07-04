use sysinfo::{ Uid};

pub fn option_to_string(option: Option<&str>) -> &str{
    match option {
        Some(s) => s.clone(),
        None => "unknown",
    }
}  

pub fn option_to_uid(option: Option<&Uid>) -> Uid {
    match option {
        Some(s) => s.clone(),
        None => todo!(),
    }
}

pub fn convert_second_to_humantime(seconds: u64) -> String{
    let mut seconds = seconds;
    let mut minutes = 0;
    let mut hours = 0;
    let mut days = 0;
    let mut years = 0;
    let mut months = 0;
    let mut weeks = 0;
    let mut result = String::new();
    if seconds >= 60 {
        minutes = seconds / 60;
        seconds = seconds % 60;
    }
    if minutes >= 60 {
        hours = minutes / 60;
        minutes = minutes % 60;
    }
    if hours >= 24 {
        days = hours / 24;
        hours = hours % 24;
    }
    if days >= 7 {
        weeks = days / 7;
        days = days % 7;
    }
    if weeks >= 4 {
        months = weeks / 4;
        weeks = weeks % 4;
    }
    if months >= 12 {
        years = months / 12;
        months = months % 12;
    }
    if years > 0 {
        result.push_str(&format!("{} years ", years));
    }
    if months > 0 {
        result.push_str(&format!("{} months ", months));
    }
    if weeks > 0 {
        result.push_str(&format!("{} weeks ", weeks));
    }
    if days > 0 {
        result.push_str(&format!("{} days ", days));
    }
    if hours > 0 {
        result.push_str(&format!("{} hours ", hours));
    }
    if minutes > 0 {
        result.push_str(&format!("{} minutes ", minutes));
    }
    if seconds > 0 {
        result.push_str(&format!("{} seconds ", seconds));
    }
    return result;
}
