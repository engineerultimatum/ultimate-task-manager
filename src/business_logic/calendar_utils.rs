use std::time::{SystemTime, UNIX_EPOCH};

/// Get the number of days in a given month
pub fn get_days_in_month(month: i32, year: i32) -> i32 {
    match month {
        0 | 2 | 4 | 6 | 7 | 9 | 11 => 31,
        1 => if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 },
        3 | 5 | 8 | 10 => 30,
        _ => 31,
    }
}

/// Get the first day of week for a given month (0 = Sunday)
pub fn get_first_day_of_week(month: i32, year: i32) -> i32 {
    // Zeller's congruence algorithm
    let m = if month < 2 { month + 13 } else { month + 1 };
    let y = if month < 2 { year - 1 } else { year };
    let q = 1;
    
    let k = y % 100;
    let j = y / 100;
    
    let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
    let day = (h + 5) % 7;
    
    if day < 0 { day + 7 } else { day }
}

/// Format month number to month name
pub fn format_month(month: i32) -> &'static str {
    match month {
        0 => "January",
        1 => "February",
        2 => "March",
        3 => "April",
        4 => "May",
        5 => "June",
        6 => "July",
        7 => "August",
        8 => "September",
        9 => "October",
        10 => "November",
        11 => "December",
        _ => "Unknown",
    }
}

/// Create a deadline timestamp (Unix seconds) for end of day
pub fn create_deadline_timestamp(year: i32, month: i32, day: i32) -> u64 {
    // Calculate total days from 1970 to the target date
    let mut total_days: i64 = 0;
    
    // Add days for each year from 1970 to target year
    for y in 1970..year {
        if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) {
            total_days += 366;
        } else {
            total_days += 365;
        }
    }
    
    // Add days for each month in the target year
    for m in 0..month {
        total_days += get_days_in_month(m, year) as i64;
    }
    
    // Add the day of month
    total_days += day as i64;
    
    // Convert to seconds, set time to end of day (23:59:59)
    let timestamp = (total_days * 86400i64 - 1) as u64;
    timestamp
}

/// Format the time remaining until a deadline
pub fn format_time_remaining(deadline: u64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    if now >= deadline {
        "EXPIRED".to_string()
    } else {
        let remaining = deadline - now;
        let days = remaining / 86400;
        let hours = (remaining % 86400) / 3600;
        let minutes = (remaining % 3600) / 60;
        
        if days > 0 {
            format!("{}d {}h left", days, hours)
        } else if hours > 0 {
            format!("{}h {}m left", hours, minutes)
        } else {
            format!("{}m left", minutes)
        }
    }
}

/// Generate calendar grid with current and adjacent month days
pub fn generate_calendar_days(month: i32, year: i32, first_day_of_week: i32, days_in_month: i32) -> Vec<(i32, bool)> {
    let mut days = vec![];
    
    // Get previous month info
    let (prev_month, prev_year) = if month == 0 {
        (11, year - 1)
    } else {
        (month - 1, year)
    };
    let prev_days_in_month = get_days_in_month(prev_month, prev_year);
    
    // Add days from previous month
    let start_day = prev_days_in_month - first_day_of_week + 1;
    for day in start_day..=prev_days_in_month {
        days.push((day, false));
    }
    
    // Add days of current month
    for day in 1..=days_in_month {
        days.push((day, true));
    }
    
    // Add days from next month to fill the grid (6x7 = 42 days)
    let remaining = 42 - days.len() as i32;
    for day in 1..=remaining {
        days.push((day, false));
    }
    
    days
}

/// Get current year and month from system time
pub fn get_current_month() -> (i32, i32) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let days_since_epoch = now / 86400;
    
    // Calculate year accounting for leap years
    let mut year = 1970i32;
    let mut remaining_days = days_since_epoch as i32;
    
    loop {
        let days_in_year = if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }
    
    // Calculate month from remaining days
    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let month_days = if is_leap {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    
    let mut month = 0;
    for (i, &days) in month_days.iter().enumerate() {
        if remaining_days < days {
            month = i as i32;
            break;
        }
        remaining_days -= days;
    }
    
    (year, month)
}
