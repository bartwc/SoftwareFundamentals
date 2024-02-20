use error::MainError;
use tudelft_dsmr_output_generator::current_over_time::{CurrentData, CurrentOverTime};
use tudelft_dsmr_output_generator::energy_over_time::{EnergyData, EnergyOverTime};
use tudelft_dsmr_output_generator::water_over_time::{WaterData, WaterOverTime};
use tudelft_dsmr_output_generator::gas_over_time::{GasData, GasOverTime};
use tudelft_dsmr_output_generator::voltage_over_time::{create_voltage_over_time_graph, VoltageData,};
use std::io::{Read};
// use std::fmt;
use derive_more::Display;
// use std::process::{id, exit};
use tudelft_dsmr_output_generator::{GraphBuilder, Graphs, date_to_timestamp};
// use tudelft_dsmr_output_generator::date_to_timestamp;

/// Contains `MainError`, and code to convert `PlotError` and `io::Error` into a `MainError`
/// https://docs.rs/tudelft-dsmr-output-generator/0.1.3/tudelft_dsmr_output_generator/index.html
mod error;
mod test;

#[derive(Debug, Display)]
#[derive(PartialEq)]
enum Versions {
    V10,
    V12,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Extensions {
    Gas,
    Recursive,
    GasRecursive,
}
#[derive(Debug)]
#[derive(PartialEq)]

enum Keys {
    Start, // 1.1.0
    Date, // 2.1
    End, // 1.2.0
    EventLogSeverity, // 3.1.n
    EventLogMessage, // 3.2.n
    EventLogDate, // 3.3.n
    InformationType, // 4,1
    GasModel, // 5.1
    GasConsumption, // 5.2
    WaterConsumption, // 6.1
    VoltageP1, // 7.1.1
    VoltageP2, // 7.1.2
    VoltageP3, // 7.1.3
    CurrentP1, // 7.2.1
    CurrentP2, // 7.2.2
    CurrentP3, // 7.2.3
    PowerP1, // 7.3.1
    PowerP2, // 7.3.2
    PowerP3, // 7.3.3
    EnergyConsumption, // 7.4.1
    EnergyProduction, // 7.4.2
    LineBreak, // NA
    ChildTelegram1, // 1.1.1
    // ChildTelegram2, // 1.1.2
    // ChildTelegram3, // 1.1.3
    EndChildTelegram1, // 1.2.1
    // EndChildTelegram2, // 1.2.2
    // EndChildTelegram3, // 1.2.3
}
// struct Header {
//     version: Option<Versions>, // None if no header found yet.
//     extensions: Vec<Extensions> 
// }
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
struct Telegram{
    telegram_version: String, // 0.0 Option<Vec<String>>
    telegram_extensions: String, // 0.0+ Option<Vec<String>>
    telegram_start: Vec<String>, // 1.1
    telegram_date: Vec<String>, // 2.1
    event_log_severity: Vec<String>, // 3.1.
    event_log_message: Vec<String>, // 3.2.n
    event_log_date: Vec<String>, // 3.3.n
    information_type: Vec<String>, // 4.1
    gas_model: Vec<String>, // 5.1
    gas_consumption: Vec<f64>, // 5.2
    water_consumption: Vec<u64>, // 6.1
    voltage_p1: Vec<f64>, // 7.1.1
    voltage_p2: Vec<f64>, // 7.1.2
    voltage_p3: Vec<f64>, // 7.1.3
    current_p1: Vec<f64>, // 7.2.1
    current_p2: Vec<f64>, // 7.2.2
    current_p3: Vec<f64>, // 7.2.3
    power_p1: Vec<f64>, // 7.3.1
    power_p2: Vec<f64>, // 7.3.2
    power_p3: Vec<f64>, // 7.3.3
    energy_consumption: Vec<f64>, // 7.4.1
    energy_production: Vec<f64>, // 7.4.2
    telegram_end: Vec<String>, // 1.2.0
    time_stamp: Vec<i64>, // 2.1.n
    child_telegram1: ChildTelegram1, // 1.1.1
    // ChildTelegram2: ChildTelegram2, // 1.1.2
    // ChildTelegram3: ChildTelegram3, // 1.1.3
}
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
struct ChildTelegram1 {
    // telegram_version: String, // 0.0
    // telegram_extensions: String, // 0.0+
    telegram_start: Vec<String>, // 1.1.1
    telegram_date: Vec<String>, // 2.1
    // event_log_severity: Vec<String>, // 3.1.n
    // event_log_message: Vec<String>, // 3.2.n
    // event_log_date: Vec<String>, // 3.3.n
    // information_type: Vec<String>, // 4.1
    gas_model: Vec<String>, // 5.1
    gas_consumption: Vec<f64>, // 5.2
    // water_consumption: Vec<u64>, // 6.1
    // voltage_p1: Vec<f64>, // 7.1.1
    // voltage_p2: Vec<f64>, // 7.1.2
    // voltage_p3: Vec<f64>, // 7.1.3
    // current_p1: Vec<f64>, // 7.2.1
    // current_p2: Vec<f64>, // 7.2.2
    // current_p3: Vec<f64>, // 7.2.3
    // power_p1: Vec<f64>, // 7.3.1
    // power_p2: Vec<f64>, // 7.3.2
    // power_p3: Vec<f64>, // 7.3.3
    // energy_consumption: Vec<f64>, // 7.4.1
    // energy_production: Vec<f64>, // 7.4.2
    telegram_end: Vec<String>, // 1.2.1
    // time_stamp: Vec<i64>, // 2.1.n
}

fn telegram_ver(telegram_version: &str) -> Result<Versions, MainError> {
    match telegram_version {
        "10" => Ok(Versions::V10),
        "12" => Ok(Versions::V12),
        _ => Err(MainError::VersionError("Neither Version 10 or 12".to_string())),
    }
}
fn version_ext(version_extension: &str) -> Result<Extensions, &'static str> {
    match version_extension {
        "g\r" => Ok(Extensions::Gas),
        "r\r" => Ok(Extensions::Recursive),
        "gr" => Ok(Extensions::GasRecursive),
        _ => Err("Invalid Version Extension or No Version Extension"),
    }
}
fn process_lines(lines: &str) -> (String, String) {
    let parts: Vec<&str> = lines.split('#').collect();
    if parts.len() == 2 {
        let telegram_id = parts[0].to_string();
        let payload = parts[1].trim_matches(|c| c == '(' || c == ')').to_string();
        (telegram_id, payload)
    } else {
        ("LineBreak".to_string(), "".to_string())
    }
}
fn version_key(version_key: &str) -> Result<Keys, String> {
    match version_key {
        "1.1.0" => Ok(Keys::Start),
        "2.1" => Ok(Keys::Date),
        "1.2.0" => Ok(Keys::End),
        key if key.starts_with("3.1.") => Ok(Keys::EventLogSeverity),
        key if key.starts_with("3.2.") => Ok(Keys::EventLogMessage),
        key if key.starts_with("3.3.") => Ok(Keys::EventLogDate),
        "4.1" => Ok(Keys::InformationType),
        "5.1" => Ok(Keys::GasModel),
        "5.2" => Ok(Keys::GasConsumption),
        "6.1" => Ok(Keys::WaterConsumption),
        "7.1.1" => Ok(Keys::VoltageP1),
        "7.1.2" => Ok(Keys::VoltageP2),
        "7.1.3" => Ok(Keys::VoltageP3),
        "7.2.1" => Ok(Keys::CurrentP1),
        "7.2.2" => Ok(Keys::CurrentP2),
        "7.2.3" => Ok(Keys::CurrentP3),
        "7.3.1" => Ok(Keys::PowerP1),
        "7.3.2" => Ok(Keys::PowerP2),
        "7.3.3" => Ok(Keys::PowerP3),
        "7.4.1" => Ok(Keys::EnergyConsumption),
        "7.4.2" => Ok(Keys::EnergyProduction),
        "LineBreak" => Ok(Keys::LineBreak),
        "1.1.1" => Ok(Keys::ChildTelegram1),
        // "1.1.2" => Ok(Keys::ChildTelegram2),
        // "1.1.3" => Ok(Keys::ChildTelegram3),
        "1.2.1" => Ok(Keys::EndChildTelegram1),
        // "1.2.2" => Ok(Keys::EndChildTelegram2),
        // "1.2.3" => Ok(Keys::EndChildTelegram3),
        other => Err(format!("Authentication Failed: {other}")),
        // _ => Err("Authentication Failed"),
    }
}
fn handle_gas_model(payload: &str, child_telegram1: &mut ChildTelegram1, telegram: &mut Telegram, child_telegram: &str) {
    if child_telegram == "100" {
        child_telegram1.gas_model.push(payload.to_string());
    } else {
        telegram.gas_model.push(payload.to_string());
    }
}
fn handle_gas_consumption(parsed_value: f64, last_gas_model: &str, child_telegram1: &mut ChildTelegram1, telegram: &mut Telegram, child_telegram: &str) {
    match last_gas_model {
        "G4" => {
            if child_telegram == "100" {
                child_telegram1.gas_consumption.push(parsed_value * 1.0);
            } else {
                telegram.gas_consumption.push(parsed_value * 1.0);
            }
        }
        "G5" => {
            if child_telegram == "100" {
                child_telegram1.gas_consumption.push(parsed_value * 10.0);
            } else {
                telegram.gas_consumption.push(parsed_value * 10.0);
            }
        }
        "G6" => {
            if child_telegram == "100" {
                child_telegram1.gas_consumption.push(parsed_value * 100.0);
            } else {
                telegram.gas_consumption.push(parsed_value * 100.0);
            }
        }
        _ => {
            println!("Random Non-gas_consumption Value: {:?}", parsed_value);
            std::process::exit(42); // Exit the program when needed
            // Handle the error as needed
        }
    }
}
fn handle_water(payload: &str, telegram: &mut Telegram) {
    match payload.trim_end_matches("*L").parse::<u64>() {
        Ok(parsed_value) => telegram.water_consumption.push(parsed_value),
        Err(_) => {
            println!("Failed to parse water_consumption: {}", payload);
            std::process::exit(42);
        }
    }
}
fn handle_voltage(payload: &str, telegram: &mut Telegram, key: &str) {
    // Implement the logic for handling voltage payload here
    // For example, you might want to parse the payload and update the Telegram struct

    // Placeholder logic:
    if let Ok(parsed_value) = payload.trim_end_matches("*V").parse::<f64>() {
        match key {
            "voltage_p1" => telegram.voltage_p1.push(parsed_value),
            "voltage_p2" => telegram.voltage_p2.push(parsed_value),
            "voltage_p3" => telegram.voltage_p3.push(parsed_value),
            _ => {
                // Handle the case where an unexpected key is provided
                println!("Unexpected key for voltage payload: {}", key);
                std::process::exit(42); // Exit the program when needed
            }
        }
    } else {
        // Handle the case where parsing fails
        println!("Failed to parse voltage payload: {}", payload);
        std::process::exit(42); // Exit the program when needed
    }
}
fn handle_current(payload: &str, telegram: &mut Telegram, key: &str) {
    // Implement the logic for handling current payload here
    // For example, you might want to parse the payload and update the Telegram struct

    // Placeholder logic:
    if let Ok(parsed_value) = payload.trim_end_matches("*A").parse::<f64>() {
        match key {
            "current_p1" => telegram.current_p1.push(parsed_value),
            "current_p2" => telegram.current_p2.push(parsed_value),
            "current_p3" => telegram.current_p3.push(parsed_value),
            _ => {
                // Handle the case where an unexpected key is provided
                println!("Unexpected key for current payload: {}", key);
                std::process::exit(42); // Exit the program when needed
            }
        }
    } else {
        // Handle the case where parsing fails
        println!("Failed to parse current payload: {}", payload);
        std::process::exit(42); // Exit the program when needed
    }
}
fn handle_power(payload: &str, telegram: &mut Telegram, key: &str) {
    // Implement the logic for handling power payload here
    // For example, you might want to parse the payload and update the Telegram struct

    // Placeholder logic:
    if let Ok(parsed_value) = payload.trim_end_matches("*kW").parse::<f64>() {
        match key {
            "power_p1" => telegram.power_p1.push(parsed_value),
            "power_p2" => telegram.power_p2.push(parsed_value),
            "power_p3" => telegram.power_p3.push(parsed_value),
            _ => {
                // Handle the case where an unexpected key is provided
                println!("Unexpected key for power payload: {}", key);
                std::process::exit(42); // Exit the program when needed
            }
        }
    } else {
        // Handle the case where parsing fails
        println!("Failed to parse power payload: {}", payload);
        std::process::exit(42); // Exit the program when needed
    }
}
fn handle_energy(payload: &str, telegram: &mut Telegram, key: &str) {
    // Implement the logic for handling energy payload here
    // For example, you might want to parse the payload and update the Telegram struct

    // Placeholder logic:
    if let Ok(parsed_value) = payload.trim_end_matches("*kWh").parse::<f64>() {
        match key {
            "energy_consumption" => telegram.energy_consumption.push(parsed_value),
            "energy_production" => telegram.energy_production.push(parsed_value),
            _ => {
                // Handle the case where an unexpected key is provided
                println!("Unexpected key for energy payload: {}", key);
                std::process::exit(42); // Exit the program when needed
            }
        }
    } else {
        // Handle the case where parsing fails
        println!("Failed to parse energy payload: {}", payload);
        std::process::exit(42); // Exit the program when needed
    }
}
fn hex_string(hex_str: &str) -> String {
    let mut string = String::new();
    let mut chars = hex_str.chars().peekable();

    while let Some(c1) = chars.next() {
        if let Some(c2) = chars.next() {
            let combined = format!("{}{}", c1, c2);
            if let Ok(byte) = u8::from_str_radix(&combined, 16) {
                string.push(byte as char);
            } else {
                // Handle invalid hexadecimal characters here if needed
            }
        } else {
            // Handle an odd number of characters if needed
        }
    }
    string
}
fn parse_datetime(input: &str) -> Option<(u16, u8, u8, u8, u8, u8, bool)> {
    // Split the input by whitespace
    let parts: Vec<&str> = input.split_whitespace().collect();
    // Ensure we have enough parts to proceed
    if parts.len() != 3 {
        return None;
    }
    
    // Extract individual components
    let date_parts: Vec<&str> = parts[0].split('-').collect();
    if date_parts.len() != 3 {
        return None;
    }
    let year_zero = date_parts[0].parse::<u16>().ok()?;
    let year = year_zero + 2000;
    let day = date_parts[2].parse::<u8>().ok()?;
    let month = match date_parts[1] {
        "Jan" => 1, "Feb" => 2, "Mar" => 3, "Apr" => 4, "May" => 5, "Jun" => 6, "Jul" => 7, "Aug" => 8, "Sep" => 9, "Oct" => 10, "Nov" => 11, "Dec" => 12, _ => return None,
    };
    let time_parts: Vec<&str> = parts[1].split(':').collect();
    let hour = time_parts[0].parse::<u8>().ok()?;
    let minute = time_parts[1].parse::<u8>().ok()?;
    let seconds = time_parts[2].parse::<u8>().ok()?;
    let dst = parts[2].trim_matches(|c| c == '(' || c == ')') == "S";
    Some((year, month, day, hour, minute, seconds, dst))
}
fn parse_dates_and_timestamps(telegram: &mut Telegram) {
    let parsed_dates: Vec<Option<(u16, u8, u8, u8, u8, u8, bool)>> = telegram
        .telegram_date.iter().map(|date| parse_datetime(date)).collect();

    // Iterate through the parsed dates and update the timestamps in the Telegram struct
    for parsed_date in parsed_dates {
        match parsed_date {
            Some((year, month, day, hour, minute, seconds, dst)) => {
                let time_stamp = date_to_timestamp(year, month, day, hour, minute, seconds, dst).unwrap_or_default();
                telegram.time_stamp.push(time_stamp);
                let _result = date_to_timestamp(year, month, day, hour, minute, seconds, dst);
            }
            None => {
                println!("Failed to parse date and time.");
                std::process::exit(42);
            }
        }
    }
}
fn vector_voltage(voltage_p1: Vec<f64>, voltage_p2: Vec<f64>, voltage_p3: Vec<f64>, time_stamp: Vec<i64>) -> Vec<VoltageData> {
    let mut voltage_data = Vec::new();

    // Iterate over the indices of the voltage_p1, voltage_p2, voltage_p3, and time_stamp vectors
    for i in 0..voltage_p1.len().min(voltage_p2.len()).min(voltage_p3.len()).min(time_stamp.len()) {
        let phase_1 = voltage_p1[i];
        let phase_2 = voltage_p2[i];
        let phase_3 = voltage_p3[i];
        let timestamp = time_stamp[i];

        // Create a new VoltageData instance and push it into the result vector
        let data = VoltageData {
            phase_1,
            phase_2,
            phase_3,
            timestamp,
        };
        voltage_data.push(data);
    }
    // println!("Voltage Data - {:#?}", voltage_data);
    voltage_data
}
fn vector_current(current_p1: Vec<f64>, current_p2: Vec<f64>, current_p3: Vec<f64>, time_stamp: Vec<i64>) -> Vec<CurrentData> {
    let mut current_data: Vec<CurrentData> = Vec::new();

    // Iterate over the indices of the current_p1, current_p2, current_p3, and time_stamp vectors
    for i in 0..current_p1.len().min(current_p2.len()).min(current_p3.len()).min(time_stamp.len()) {
        let phase_1 = current_p1[i];
        let phase_2 = current_p2[i];
        let phase_3 = current_p3[i];
        let timestamp = time_stamp[i];

        // Create a new CurrentData instance and push it into the result vector
        let data = CurrentData {
            phase_1,
            phase_2,
            phase_3,
            timestamp,
        };
        current_data.push(data);
    }
    // println!("Current Data - {:#?}", current_data);
    current_data
}
fn vector_energy(energy_produced: Vec<f64>, energy_consumed: Vec<f64>, time_stamp: Vec<i64>) -> Vec<EnergyData> {
    let mut energy_data: Vec<EnergyData> = Vec::new();
    let mut produced_diff = Vec::new();
    let mut consumed_diff = Vec::new();
    let mut time_diff = Vec::new();

    // Iterate over the indices of the energy_consumption, energy_production, and time_stamp vectors
    for i in 0..energy_produced.len().min(energy_consumed.len()).min(time_stamp.len()) {
        if i == 0 {
            produced_diff.push(0.0);
            consumed_diff.push(0.0);
            time_diff.push(time_stamp[i]);
        }
        else if i < 12 {
            produced_diff.push(energy_produced[i] - energy_produced[i-1]);
            consumed_diff.push(energy_consumed[i] - energy_consumed[i-1]);
            time_diff.push(time_stamp[i]);
        }
        else {
            produced_diff.remove(0);
            produced_diff.push(energy_produced[i] - energy_produced[i-1]);
            consumed_diff.remove(0);
            consumed_diff.push(energy_consumed[i] - energy_consumed[i-1]);
            time_diff.remove(0);
            time_diff.push(time_stamp[i]);
        }
        // println!("Produced - {:?}", produced_diff);
        // println!("Consumed - {:?}", consumed_diff);
        // println!("Time_Stamp - {:?}", time_diff);
    }
    // println!{"{:#?}", energy_data};
    for i in 0..produced_diff.len() {
        let data = EnergyData {
            produced: produced_diff[i], // Choose the specific element
            consumed: consumed_diff[i], // Choose the specific element
            timestamp: time_diff[i],
        };
        energy_data.push(data);
    }
    // println!("Energy Data - {:#?}", energy_data);
    energy_data
}
fn vector_water(water_consumed: Vec<u64>, time_stamp: Vec<i64>) -> Vec<WaterData> {
    let mut water_data: Vec<WaterData> = Vec::new();
    let mut consumed_diff = Vec::new();
    let mut time_diff = Vec::new();

    // Iterate over the indices of the water_consumption, and time_stamp vectors
    for i in 0..water_consumed.len().min(time_stamp.len()) {
        if i == 0 {
            consumed_diff.push(0);
            time_diff.push(0);
        }
        else {
            consumed_diff.push(water_consumed[i] - water_consumed[i-1]);
            time_diff.push(time_stamp[i]);
        }
    }
    for i in 0..consumed_diff.len() {
        let data = WaterData {
            water_delta: consumed_diff[i],
            timestamp: time_stamp[i],
        };
        water_data.push(data);
    }
    // println!("Water Data - {:#?}", water_data);
    water_data
}
fn vector_gas(gas_consumed: Vec<f64>, time_stamp: Vec<i64>) -> Vec<GasData> {
    let mut gas_data: Vec<GasData> = Vec::new();
    let mut consumed_diff = Vec::new();
    let mut time_diff = Vec::new();

    // Iterate over the indices of the gas_consumption, and time_stamp vectors
    for i in 0..gas_consumed.len().min(time_stamp.len()) {
        if i == 0 {
            consumed_diff.push(0.0);
            time_diff.push(0);
        }
        else {
            consumed_diff.push(gas_consumed[i] - gas_consumed[i-1]);
            time_diff.push(time_stamp[i]);
        }
    }
    for i in 0..consumed_diff.len() {
        let data = GasData {
            gas_delta: consumed_diff[i],
            timestamp: time_stamp[i],
        };
        gas_data.push(data);
    }
    // println!("Gas Data - {:#?}", gas_data);
    gas_data
}
fn parse(input: &str) -> Result<Telegram, MainError> {
    // cd User/Y1S1-SoftwareFundamentals/2023-09-12_Proj-Individual/btee
    // Note that you can use this function:
    // tudelft_dsmr_output_generator::date_to_timestamp(year, month, day, hour, minute, seconds, dst)
    // let l = lines.len(); // print!("{}",l); // print!("{}",input);

    let mut telegram: Telegram = Default::default(); // Initialize with default values

    let mut child_telegram = "000";
    let mut child_telegram1: ChildTelegram1 = Default::default(); // Initialize with default values

    let telegram_version = &input[2..4].to_string();
    println!("{:?}", telegram_version);

    let telegram_version = telegram_ver(telegram_version).expect("Please Check");
    // set version here if validation is successful
    match telegram_version {
        Versions::V10 | Versions::V12 => {
            telegram.telegram_version = telegram_version.to_string();
        },
    }

    // match telegram_ver(telegram_version) {
    //     Ok(Versions::V10) => telegram.telegram_version = telegram_version.to_string(), // println!("Version - {:?}", telegram_version),
    //     Ok(Versions::V12) => telegram.telegram_version = telegram_version.to_string(), // println!("Version - {:?}", telegram_version),
    //     _ => {
    //         println!("Neither Version 12 nor 10");
    //         std::process::exit(42); // Exit the program when needed
    //     }
    // };

    let version_extension = &input[6..8].to_string();
    // println!("{:?}", version_extension);
    match version_ext(version_extension) {
        Ok(Extensions::Gas) => telegram.telegram_extensions = version_extension.to_string(),
        Ok(Extensions::Recursive) => telegram.telegram_extensions = version_extension.to_string(),
        Ok(Extensions::GasRecursive) => telegram.telegram_extensions = version_extension.to_string(),
        _ => println!("Neither Gas, Recursive nor Both"),
    }
    let lines: Vec<&str> = input.lines().collect();
    // let current_GasModel = "";
    // println!("global - {:?}",current_GasModel);
    for line in lines {
        // println!("{}", line);
        let (telegram_id, payload) = process_lines(line);
        // println!("Telegram ID: {}, Payload: {}", telegram_id, payload);
        let telegram_id_clone = telegram_id.to_string(); // Clone telegram_id
        match version_key(&telegram_id_clone) {
            Ok(Keys::Start) => telegram.telegram_start.push(payload.to_string()), // println!("Start - {:?}", payload),
            Ok(Keys::Date) => {
                if child_telegram == "100" {
                    child_telegram1.telegram_date.push(payload.to_string());
                // } else if child_telegram == "010" {
                //     child_telegram2.telegram_date.push(payload.to_string());
                // } else if child_telegram == "001" {
                //     child_telegram3.telegram_date.push(payload.to_string());
                } else {
                    telegram.telegram_date.push(payload.to_string());
                }
            },
            Ok(Keys::EventLogSeverity) => telegram.event_log_severity.push(payload.to_string()),
            Ok(Keys::EventLogMessage) => telegram.event_log_message.push(hex_string(&payload)),
            Ok(Keys::EventLogDate) => telegram.event_log_date.push(payload.to_string()),
            Ok(Keys::InformationType) => telegram.information_type.push(payload.to_string()),
            Ok(Keys::GasModel) => { handle_gas_model(&payload, &mut child_telegram1, &mut telegram, child_telegram); },
            Ok(Keys::GasConsumption) => match payload.trim_end_matches("*m3").parse::<f64>() {
                Ok(parsed_value) => {
                    if let Some(last_gas_model) = child_telegram1.gas_model.last().cloned() {
                        handle_gas_consumption(parsed_value, &last_gas_model, &mut child_telegram1, &mut telegram, child_telegram);
                    } else {
                        println!("Missing Parsed gas_consumption Value: {}", payload);
                        std::process::exit(42); // Exit the program when needed
                    }
                }
                _ => {
                    println!("Failed to parse gas_consumption: {}", payload);
                    std::process::exit(42); // Exit the program when needed
                }
            }
            Ok(Keys::WaterConsumption) => handle_water(&payload, &mut telegram),
            Ok(Keys::VoltageP1) => handle_voltage(&payload, &mut telegram, "voltage_p1"),
            Ok(Keys::VoltageP2) => handle_voltage(&payload, &mut telegram, "voltage_p2"),
            Ok(Keys::VoltageP3) => handle_voltage(&payload, &mut telegram, "voltage_p3"),
            Ok(Keys::CurrentP1) => handle_current(&payload, &mut telegram, "current_p1"),
            Ok(Keys::CurrentP2) => handle_current(&payload, &mut telegram, "current_p2"),
            Ok(Keys::CurrentP3) => handle_current(&payload, &mut telegram, "current_p3"),
            Ok(Keys::PowerP1) => handle_power(&payload, &mut telegram, "power_p1"),
            Ok(Keys::PowerP2) => handle_power(&payload, &mut telegram, "power_p2"),
            Ok(Keys::PowerP3) => handle_power(&payload, &mut telegram, "power_p3"),
            Ok(Keys::EnergyConsumption) => handle_energy(&payload, &mut telegram, "energy_consumption"),
            Ok(Keys::EnergyProduction) => handle_energy(&payload, &mut telegram, "energy_production"),
            Ok(Keys::End) => telegram.telegram_end.push(payload.to_string()), // println!("End - {:?}", payload),
            Ok(Keys::LineBreak) => {
                // Do nothing
            },
            Err(e) => {
                println!("Invalid telegram. Exiting with exit code 42. {e}");
                std::process::exit(42); // Exit with code 42 for invalid telegrams
            }
            Ok(Keys::ChildTelegram1) => {
                // println!("{:?}", payload);
                child_telegram1.telegram_start.push(payload.to_string());
                child_telegram = "100";
                // println!("{:#?}",child_telegram1);
            },
            Ok(Keys::EndChildTelegram1) => {
                child_telegram1.telegram_end.push(payload.to_string());
                child_telegram = "000";
                // println!("{:#?}",child_telegram1);
            },
        };
    }
    println!("{:#?}",telegram);
    parse_dates_and_timestamps(&mut telegram);
    // println!("{:#?}",child_telegram1);

    telegram.child_telegram1 = child_telegram1;
    telegram.gas_consumption.extend(telegram.child_telegram1.gas_consumption.iter().cloned());
    // println!("{:#?}",&telegram);
    // for telegram_date in &telegram.telegram_date { println!("{}", telegram_date); }
    Ok(telegram)
}

/// Reads the DSMR file from the terminal. /// You do not need to change this nor understand this.
/// You can use /// ``` /// cargo run < examples/good/simple_gas.dsmr /// ```
/// to quickly test an example dsmr file with your submission.
/// We also use this at the end to assist with grading your submission!
fn read_from_stdin() -> Result<String, MainError> {
    let mut input = Vec::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_end(&mut input)?;
    Ok(String::from_utf8_lossy(&input).to_string())
}
fn main() -> Result<(), MainError> {

    let input = read_from_stdin()?;
    let parsed = parse(&input)?;
    let mut result = Graphs::new()?;

    // let result = tudelft_dsmr_output_generator::date_to_timestamp(year, month, day, hour, minute, seconds, dst)
    let voltage_values: Vec<VoltageData> = vector_voltage(parsed.voltage_p1.clone(), parsed.voltage_p2.clone(), parsed.voltage_p3.clone(), parsed.time_stamp.clone());
    result.add_graph(create_voltage_over_time_graph(voltage_values))?;
    
    let current_values: Vec<CurrentData> = vector_current(parsed.current_p1.clone(), parsed.current_p2.clone(), parsed.current_p3.clone(), parsed.time_stamp.clone());
    let mut current_graph = CurrentOverTime::new(); // Create an instance of CurrentOverTime
    for data_current in current_values {
        current_graph.add(data_current); // Add each CurrentData instance to the graph by moving it
    }
    let _ = result.add_graph(current_graph);

    let energy_values: Vec<EnergyData> = vector_energy(parsed.energy_production.clone(), parsed.energy_consumption.clone(), parsed.time_stamp.clone());
    let mut energy_graph = EnergyOverTime::new(); // Create an instance of EnergyOverTime
    for data_energy in energy_values {
        energy_graph.add(data_energy); // Add each EnergyData instance to the graph by moving it
    }
    let _ = result.add_graph(energy_graph);

    let water_values: Vec<WaterData> = vector_water(parsed.water_consumption.clone(), parsed.time_stamp.clone());
    let mut water_graph = WaterOverTime::new(); // Create an instance of WaterOverTime
    for data_water in water_values {
        water_graph.add(data_water); // Add each WaterData instance to the graph by moving it
    }
    let _ = result.add_graph(water_graph);

    let gas_values: Vec<GasData> = vector_gas(parsed.gas_consumption.clone(), parsed.time_stamp.clone());
    let mut gas_graph = GasOverTime::new(); // Create an instance of GasOverTime
    for data_gas in gas_values {
        gas_graph.add(data_gas); // Add each GasData instance to the graph by moving it
    }
    let _ = result.add_graph(gas_graph);

    result.generate().expect("error generating graphs");
    Ok(())
}

// sudo apt-get update && sudo apt-get install libssl-dev pkg-config cmake zlib1g-dev
// RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin
// cargo tarpaulin --out Html --all-features --output-dir target/tarpaulin
// cargo run < examples/good_sequences/should_parse_3_recursive.dsmr