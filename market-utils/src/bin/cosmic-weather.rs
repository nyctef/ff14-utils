use chrono::{Duration, TimeZone, Utc};

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
    let now = Utc::now();
    println!("Current time: {}", now);

    for i in 0..10 {
        let forecast_time = now + Duration::seconds(i * 8 * 175); // Increment by 8 Eorzean hours
        let forecast_target = calculate_forecast_target(forecast_time);

        let weather = match forecast_target {
            0..=14 => "Moon Dust",
            15..=84 => "Fair Skies",
            85..=99 => "Umbral Wind",
            _ => unreachable!(),
        };

        println!("Forecast for {}: {}", forecast_time, weather);
    }
}
