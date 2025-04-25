use chrono::{Duration, Local, Utc};

fn calculate_forecast_target(l_date: chrono::DateTime<Utc>) -> u32 {
    let unix_seconds = l_date.timestamp();
    let bell = unix_seconds / 175;
    let increment = (bell + 8 - (bell % 8)) % 24;
    let total_days = (unix_seconds / 4200) as u32;
    let calc_base = total_days.wrapping_mul(100).wrapping_add(increment as u32);
    let step1 = (calc_base << 11) ^ calc_base;
    let step2 = (step1 >> 8) ^ step1;
    step2 % 100
}

fn main() {
    let now = Local::now(); // Use local time
    println!("Current time: {}", now.format("%H:%M")); // Print time in HH:MM format

    let mut found_weathers = 0;
    let mut forecast_time = now;

    while found_weathers < 3 {
        forecast_time = forecast_time + Duration::seconds(8 * 175); // Increment by 8 Eorzean hours
        let interval_start =
            forecast_time - Duration::seconds(forecast_time.timestamp() % (8 * 175)); // Align to the start of the interval
        let forecast_target = calculate_forecast_target(interval_start.with_timezone(&Utc)); // Convert to UTC for calculation

        let weather = match forecast_target {
            0..=14 => "Moon Dust",
            15..=84 => "Fair Skies",
            85..=99 => "Umbral Wind",
            _ => unreachable!(),
        };

        if weather != "Fair Skies" {
            let relative_duration = interval_start - now;
            let hours = relative_duration.num_hours();
            let minutes = relative_duration.num_minutes() % 60;
            println!(
                "Next weather: {} at {} (in {}h{}m)",
                weather,
                interval_start.format("%H:%M"),
                hours,
                minutes
            );
            found_weathers += 1;
        }
    }
}
