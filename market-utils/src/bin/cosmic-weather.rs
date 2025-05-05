use chrono::{Duration, Local, Utc};

fn calculate_forecast_target(l_date: chrono::DateTime<Utc>) -> u32 {
    // based on https://github.com/xivapi/ffxiv-datamining/blob/master/docs/Weather.md
    let unix_seconds = l_date.timestamp();
    let bell = unix_seconds / 175;
    let increment = (bell + 8 - (bell % 8)) % 24;
    let total_days = (unix_seconds / 4200) as u32;
    let calc_base = total_days.wrapping_mul(100).wrapping_add(increment as u32);
    let step1 = (calc_base << 11) ^ calc_base;
    let step2 = (step1 >> 8) ^ step1;
    step2 % 100
}

fn format_time(time: chrono::DateTime<Utc>) -> String {
    let local_time = time.with_timezone(&Local);
    local_time.format("%H:%M").to_string()
}

fn format_interval(d: Duration) -> String {
    let hours = d.num_hours();
    let minutes = d.num_minutes() % 60;
    if hours > 0 {
        format!("{}h{}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

fn predict_weather(interval_start: chrono::DateTime<Utc>) -> &'static str {
    let forecast_target = calculate_forecast_target(interval_start);

    // data from https://nekobot.io/ffxiv/time
    let weather = match forecast_target {
        0..=14 => "Moon Dust",
        15..=84 => "Fair Skies",
        85..=99 => "Umbral Wind",
        _ => unreachable!(),
    };
    weather
}

fn main() {
    let now = Utc::now();
    println!("Current time: {}", format_time(now));
    let current_weather = predict_weather(now);
    let interval_end = now + Duration::seconds(8 * 175 - (now.timestamp() % (8 * 175)));
    let time_until_end = interval_end - now;

    println!(
        "Current weather: {} (ends at {} in {})",
        current_weather,
        format_time(interval_end),
        format_interval(time_until_end)
    );

    let mut found = 0;
    let mut forecast_time = now;

    while found < 3 {
        // Increment by 8 Eorzean hours
        forecast_time = forecast_time + Duration::seconds(8 * 175);
        // Align to the start of the interval
        let interval_start =
            forecast_time - Duration::seconds(forecast_time.timestamp() % (8 * 175));
        let weather = predict_weather(interval_start);

        if weather != "Fair Skies" {
            let time_until = interval_start - now;
            println!(
                "Next weather event: {} at {} (in {})",
                weather,
                format_time(interval_start),
                format_interval(time_until)
            );
            found += 1;
        }
    }
}
