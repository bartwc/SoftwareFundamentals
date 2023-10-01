use error::MainError;
use tudelft_dsmr_output_generator::current_over_time::{CurrentData, CurrentOverTime};
use tudelft_dsmr_output_generator::energy_over_time::{self, EnergyData, EnergyOverTime};
use tudelft_dsmr_output_generator::water_over_time::{self, WaterData, WaterOverTime};
use tudelft_dsmr_output_generator::gas_over_time::{self, GasData, GasOverTime};
use tudelft_dsmr_output_generator::voltage_over_time::{create_voltage_over_time_graph, VoltageData,};
use std::io::{Read};
use std::process::{id, exit};
use tudelft_dsmr_output_generator::{GraphBuilder, Graphs, date_to_timestamp, UnixTimeStamp};
/// Contains `MainError`, and code to convert `PlotError` and `io::Error` into a `MainError`
/// https://docs.rs/tudelft-dsmr-output-generator/0.1.3/tudelft_dsmr_output_generator/index.html
mod error;

enum Versions {
    V10,
    V12,
}
enum Extensions {
    Gas,
    Recursive,
    GasRecursive,
}
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
    ChildTelegram2, // 1.1.2
    ChildTelegram3, // 1.1.3  
    EndChildTelegram1, // 1.2.1
    EndChildTelegram2, // 1.2.2
    EndChildTelegram3, // 1.2.3  
}
// struct Header {
//     version: Option<Versions>, // None if no header found yet.
//     extensions: Vec<Extensions> 
// }
#[derive(Debug)]
#[derive(Default)]
struct Telegram{
    Telegram_Version: String, // 0.0
    Telegram_Extensions: String, // 0.0+
    Telegram_Start: Vec<String>, // 1.1
    Telegram_Date: Vec<String>, // 2.1
    EventLogSeverity: Vec<String>, // 3.1.
    EventLogMessage: Vec<String>, // 3.2.n
    EventLogDate: Vec<String>, // 3.3.n
    InformationType: Vec<String>, // 4.1
    GasModel: Vec<String>, // 5.1
    GasConsumption: Vec<f64>, // 5.2
    WaterConsumption: Vec<u64>, // 6.1
    VoltageP1: Vec<f64>, // 7.1.1
    VoltageP2: Vec<f64>, // 7.1.2
    VoltageP3: Vec<f64>, // 7.1.3
    CurrentP1: Vec<f64>, // 7.2.1
    CurrentP2: Vec<f64>, // 7.2.2
    CurrentP3: Vec<f64>, // 7.2.3
    PowerP1: Vec<f64>, // 7.3.1
    PowerP2: Vec<f64>, // 7.3.2
    PowerP3: Vec<f64>, // 7.3.3
    EnergyConsumption: Vec<f64>, // 7.4.1
    EnergyProduction: Vec<f64>, // 7.4.2
    Telegram_End: Vec<String>, // 1.2.0
    TimeStamp: Vec<i64>, // 2.1.n
    ChildTelegram1: ChildTelegram1, // 1.1.1
    ChildTelegram2: ChildTelegram2, // 1.1.2
    ChildTelegram3: ChildTelegram3, // 1.1.3
}
#[derive(Debug)]
#[derive(Default)]
struct ChildTelegram1 {
    // Telegram_Version: String, // 0.0
    // Telegram_Extensions: String, // 0.0+
    Telegram_Start: Vec<String>, // 1.1.1
    Telegram_Date: Vec<String>, // 2.1
    EventLogSeverity: Vec<String>, // 3.1.n
    EventLogMessage: Vec<String>, // 3.2.n
    EventLogDate: Vec<String>, // 3.3.n
    InformationType: Vec<String>, // 4.1
    GasModel: Vec<String>, // 5.1
    GasConsumption: Vec<f64>, // 5.2
    WaterConsumption: Vec<u64>, // 6.1
    VoltageP1: Vec<f64>, // 7.1.1
    VoltageP2: Vec<f64>, // 7.1.2
    VoltageP3: Vec<f64>, // 7.1.3
    CurrentP1: Vec<f64>, // 7.2.1
    CurrentP2: Vec<f64>, // 7.2.2
    CurrentP3: Vec<f64>, // 7.2.3
    PowerP1: Vec<f64>, // 7.3.1
    PowerP2: Vec<f64>, // 7.3.2
    PowerP3: Vec<f64>, // 7.3.3
    EnergyConsumption: Vec<f64>, // 7.4.1
    EnergyProduction: Vec<f64>, // 7.4.2
    Telegram_End: Vec<String>, // 1.2.1
    TimeStamp: Vec<i64>, // 2.1.n
}
#[derive(Debug)]
#[derive(Default)]
struct ChildTelegram2 {
    // Telegram_Version: String, // 0.0
    // Telegram_Extensions: String, // 0.0+
    Telegram_Start: Vec<String>, // 1.1.2
    Telegram_Date: Vec<String>, // 2.1
    EventLogSeverity: Vec<String>, // 3.1.n
    EventLogMessage: Vec<String>, // 3.2.n
    EventLogDate: Vec<String>, // 3.3.n
    InformationType: Vec<String>, // 4.1
    GasModel: Vec<String>, // 5.1
    GasConsumption: Vec<f64>, // 5.2
    WaterConsumption: Vec<u64>, // 6.1
    VoltageP1: Vec<f64>, // 7.1.1
    VoltageP2: Vec<f64>, // 7.1.2
    VoltageP3: Vec<f64>, // 7.1.3
    CurrentP1: Vec<f64>, // 7.2.1
    CurrentP2: Vec<f64>, // 7.2.2
    CurrentP3: Vec<f64>, // 7.2.3
    PowerP1: Vec<f64>, // 7.3.1
    PowerP2: Vec<f64>, // 7.3.2
    PowerP3: Vec<f64>, // 7.3.3
    EnergyConsumption: Vec<f64>, // 7.4.1
    EnergyProduction: Vec<f64>, // 7.4.2
    Telegram_End: Vec<String>, // 1.2.2
    TimeStamp: Vec<i64>, // 2.1.n
}
#[derive(Debug)]
#[derive(Default)]
struct ChildTelegram3 {
    // Telegram_Version: String, // 0.0
    // Telegram_Extensions: String, // 0.0+
    Telegram_Start: Vec<String>, // 1.1.3
    Telegram_Date: Vec<String>, // 2.1
    EventLogSeverity: Vec<String>, // 3.1.n
    EventLogMessage: Vec<String>, // 3.2.n
    EventLogDate: Vec<String>, // 3.3.n
    InformationType: Vec<String>, // 4.1
    GasModel: Vec<String>, // 5.1
    GasConsumption: Vec<f64>, // 5.2
    WaterConsumption: Vec<u64>, // 6.1
    VoltageP1: Vec<f64>, // 7.1.1
    VoltageP2: Vec<f64>, // 7.1.2
    VoltageP3: Vec<f64>, // 7.1.3
    CurrentP1: Vec<f64>, // 7.2.1
    CurrentP2: Vec<f64>, // 7.2.2
    CurrentP3: Vec<f64>, // 7.2.3
    PowerP1: Vec<f64>, // 7.3.1
    PowerP2: Vec<f64>, // 7.3.2
    PowerP3: Vec<f64>, // 7.3.3
    EnergyConsumption: Vec<f64>, // 7.4.1
    EnergyProduction: Vec<f64>, // 7.4.2
    Telegram_End: Vec<String>, // 1.2.3
    TimeStamp: Vec<i64>, // 2.1.n
}
fn telegram_ver(telegram_version: &str) -> Result<Versions, &'static str> {
    match telegram_version {
        "10" => Ok(Versions::V10),
        "12" => Ok(Versions::V12),
        _ => Err("Neither Version 12 or 10"),
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
        "1.1.2" => Ok(Keys::ChildTelegram2),
        "1.1.3" => Ok(Keys::ChildTelegram3),
        "1.2.1" => Ok(Keys::EndChildTelegram1),
        "1.2.2" => Ok(Keys::EndChildTelegram2),
        "1.2.3" => Ok(Keys::EndChildTelegram3),
        other => Err(format!("Authentication Failed: {other}")),
        // _ => Err("Authentication Failed"),
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
        "Jan" => 1, "Feb" => 2, "Mar" => 3,
        "Apr" => 4, "May" => 5, "Jun" => 6,
        "Jul" => 7, "Aug" => 8, "Sep" => 9,
        "Oct" => 10, "Nov" => 11, "Dec" => 12,
        _ => return None,
    };
    let time_parts: Vec<&str> = parts[1].split(':').collect();
    let hour = time_parts[0].parse::<u8>().ok()?;
    let minute = time_parts[1].parse::<u8>().ok()?;
    let seconds = time_parts[2].parse::<u8>().ok()?;
    let dst = parts[2].trim_matches(|c| c == '(' || c == ')') == "S";
    Some((year, month, day, hour, minute, seconds, dst))
}
fn vector_voltage(voltage_p1: Vec<f64>, voltage_p2: Vec<f64>, voltage_p3: Vec<f64>, time_stamp: Vec<i64>) -> Vec<VoltageData> {
    let mut voltage_data = Vec::new();

    // Iterate over the indices of the VoltageP1, VoltageP2, VoltageP3, and TimeStamp vectors
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

    // Iterate over the indices of the CurrentP1, CurrentP2, CurrentP3, and TimeStamp vectors
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

    // Iterate over the indices of the EnergyConsumption, EnergyProduction, and TimeStamp vectors
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
    energy_data
    // println!("Energy Data - {:#?}", energy_data);
}
fn vector_water(water_consumed: Vec<u64>, time_stamp: Vec<i64>) -> Vec<WaterData> {
    let mut water_data: Vec<WaterData> = Vec::new();
    let mut consumed_diff = Vec::new();
    let mut time_diff = Vec::new();

    // Iterate over the indices of the WaterConsumption, and TimeStamp vectors
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

    // Iterate over the indices of the GasConsumption, and TimeStamp vectors
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
    // let mut telegram = Telegram {
    //     Telegram_Version: String::new(), 
    //     Telegram_Extensions: String::new(),
    //     Telegram_Start: Vec::new(),
    //     Telegram_Date: Vec::new(),
    //     EventLogSeverity: Vec::new(),
    //     EventLogMessage: Vec::new(),
    //     EventLogDate: Vec::new(),
    //     InformationType: Vec::new(),
    //     GasModel: Vec::new(),
    //     GasConsumption: Vec::new(),
    //     WaterConsumption: Vec::new(),
    //     VoltageP1: Vec::new(),
    //     VoltageP2: Vec::new(),
    //     VoltageP3: Vec::new(),
    //     CurrentP1: Vec::new(),
    //     CurrentP2: Vec::new(),
    //     CurrentP3: Vec::new(),
    //     PowerP1: Vec::new(),
    //     PowerP2: Vec::new(),
    //     PowerP3: Vec::new(),
    //     EnergyConsumption: Vec::new(),
    //     EnergyProduction: Vec::new(),
    //     Telegram_End: Vec::new(),
    //     TimeStamp: Vec::new(),
    //     ChildTelegram1: Vec::new(),
    //     ChildTelegram2: Vec::new(),
    //     ChildTelegram3: Vec::new(),
    // };
    let mut child_telegram = "000";
    let mut child_telegram1: ChildTelegram1 = Default::default(); // Initialize with default values
    // let mut child_telegram1 = ChildTelegram1 {
    //     Telegram_Start: Vec::new(),
    //     Telegram_Date: Vec::new(),
    //     EventLogSeverity: Vec::new(),
    //     EventLogMessage: Vec::new(),
    //     EventLogDate: Vec::new(),
    //     InformationType: Vec::new(),
    //     GasModel: Vec::new(),
    //     GasConsumption: Vec::new(),
    //     WaterConsumption: Vec::new(),
    //     VoltageP1: Vec::new(),
    //     VoltageP2: Vec::new(),
    //     VoltageP3: Vec::new(),
    //     CurrentP1: Vec::new(),
    //     CurrentP2: Vec::new(),
    //     CurrentP3: Vec::new(),
    //     PowerP1: Vec::new(),
    //     PowerP2: Vec::new(),
    //     PowerP3: Vec::new(),
    //     EnergyConsumption: Vec::new(),
    //     EnergyProduction: Vec::new(),
    //     Telegram_End: Vec::new(),
    //     TimeStamp: Vec::new(),
    // };
    let mut child_telegram2: ChildTelegram2 = Default::default(); // Initialize with default values
    // let mut child_telegram2 = ChildTelegram2 {
    //     // Telegram_Version: String::new(),
    //     Telegram_Start: Vec::new(),
    //     Telegram_Date: Vec::new(),
    //     EventLogSeverity: Vec::new(),
    //     EventLogMessage: Vec::new(),
    //     EventLogDate: Vec::new(),
    //     InformationType: Vec::new(),
    //     GasModel: Vec::new(),
    //     GasConsumption: Vec::new(),
    //     WaterConsumption: Vec::new(),
    //     VoltageP1: Vec::new(),
    //     VoltageP2: Vec::new(),
    //     VoltageP3: Vec::new(),
    //     CurrentP1: Vec::new(),
    //     CurrentP2: Vec::new(),
    //     CurrentP3: Vec::new(),
    //     PowerP1: Vec::new(),
    //     PowerP2: Vec::new(),
    //     PowerP3: Vec::new(),
    //     EnergyConsumption: Vec::new(),
    //     EnergyProduction: Vec::new(),
    //     Telegram_End: Vec::new(),
    //     TimeStamp: Vec::new(),
    // };
    let mut child_telegram3: ChildTelegram3 = Default::default(); // Initialize with default values
    // let mut child_telegram3 = ChildTelegram3 {
    //     // Telegram_Version: String::new(),
    //     Telegram_Start: Vec::new(),
    //     Telegram_Date: Vec::new(),
    //     EventLogSeverity: Vec::new(),
    //     EventLogMessage: Vec::new(),
    //     EventLogDate: Vec::new(),
    //     InformationType: Vec::new(),
    //     GasModel: Vec::new(),
    //     GasConsumption: Vec::new(),
    //     WaterConsumption: Vec::new(),
    //     VoltageP1: Vec::new(),
    //     VoltageP2: Vec::new(),
    //     VoltageP3: Vec::new(),
    //     CurrentP1: Vec::new(),
    //     CurrentP2: Vec::new(),
    //     CurrentP3: Vec::new(),
    //     PowerP1: Vec::new(),
    //     PowerP2: Vec::new(),
    //     PowerP3: Vec::new(),
    //     EnergyConsumption: Vec::new(),
    //     EnergyProduction: Vec::new(),
    //     Telegram_End: Vec::new(),
    //     TimeStamp: Vec::new(),
    // };
    let telegram_version = &input[2..4].to_string();
    match telegram_ver(&telegram_version) {
        Ok(Versions::V10) => telegram.Telegram_Version = telegram_version.to_string(), // println!("Version - {:?}", telegram_version),
        Ok(Versions::V12) => telegram.Telegram_Version = telegram_version.to_string(), // println!("Version - {:?}", telegram_version),
        _ => {
            println!("Neither Version 12 nor 10");
            std::process::exit(42); // Exit the program when needed
        }
    };
    let version_extension = &input[6..8].to_string();
    // println!("{:?}", version_extension);
    match version_ext(&version_extension) {
        Ok(Extensions::Gas) => telegram.Telegram_Extensions = version_extension.to_string(),
        Ok(Extensions::Recursive) => telegram.Telegram_Extensions = version_extension.to_string(),
        Ok(Extensions::GasRecursive) => telegram.Telegram_Extensions = version_extension.to_string(),
        _ => println!("Neither Gas, Recursive nor Both"),
    }
    let lines: Vec<&str> = input.lines().collect();
    // let current_gas_model = "";
    // println!("global - {:?}",current_gas_model);
    for line in lines {
        // println!("{}", line);
        let (telegram_id, payload) = process_lines(line);
        // println!("Telegram ID: {}, Payload: {}", telegram_id, payload);
        let telegram_id_clone = telegram_id.to_string(); // Clone telegram_id
        match version_key(&telegram_id_clone) {
            Ok(Keys::Start) => telegram.Telegram_Start.push(payload.to_string()), // println!("Start - {:?}", payload),
            Ok(Keys::Date) => {
                if child_telegram == "100" {
                    child_telegram1.Telegram_Date.push(payload.to_string());
                } else if child_telegram == "010" {
                    child_telegram2.Telegram_Date.push(payload.to_string());
                } else if child_telegram == "001" {
                    child_telegram3.Telegram_Date.push(payload.to_string());
                } else {
                    telegram.Telegram_Date.push(payload.to_string());
                }
            },
            Ok(Keys::EventLogSeverity) => telegram.EventLogSeverity.push(payload.to_string()),
            Ok(Keys::EventLogMessage) => telegram.EventLogMessage.push(hex_string(&payload)),
            Ok(Keys::EventLogDate) => telegram.EventLogDate.push(payload.to_string()),
            Ok(Keys::InformationType) => telegram.InformationType.push(payload.to_string()),
            Ok(Keys::GasModel) => {
                // Set the current gas model
                // let current_gas_model = payload.to_string();
                // println!("hello - {:?}",current_gas_model);
                if child_telegram == "100" {
                    child_telegram1.GasModel.push(payload.to_string());
                } else if child_telegram == "010" {
                    child_telegram2.GasModel.push(payload.to_string());
                } else if child_telegram == "001" {
                    child_telegram3.GasModel.push(payload.to_string());
                } else {
                    telegram.GasModel.push(payload.to_string());
                }            
            },
            Ok(Keys::GasConsumption) => match payload.trim_end_matches("*m3").parse::<f64>() {
                Ok(parsed_value) => {
                    // println!("{:#?}",parsed_value);
                    // println!("bye1 - {:?}",current_gas_model);
                    // println!("bye2 - {:?}",telegram.GasModel);
                    // println!("bye3 - {:?}",child_telegram1.GasModel);
                    if let Some(last_gas_model) = child_telegram1.GasModel.last() {
                        // println!("gas - {:?}",last_gas_model);
                        match last_gas_model.as_str() {
                            "G4" => {
                                if child_telegram == "100" {
                                    child_telegram1.GasConsumption.push(parsed_value * 1.0);
                                } else if child_telegram == "010" {
                                    child_telegram2.GasConsumption.push(parsed_value * 1.0);
                                } else if child_telegram == "001" {
                                    child_telegram3.GasConsumption.push(parsed_value * 1.0);
                                } else {
                                    telegram.GasConsumption.push(parsed_value * 1.0);
                                }
                            }
                            "G5" => {
                                if child_telegram == "100" {
                                    child_telegram1.GasConsumption.push(parsed_value * 10.0);
                                } else if child_telegram == "010" {
                                    child_telegram2.GasConsumption.push(parsed_value * 10.0);
                                } else if child_telegram == "001" {
                                    child_telegram3.GasConsumption.push(parsed_value * 10.0);
                                } else {
                                    telegram.GasConsumption.push(parsed_value * 10.0);
                                }
                            }
                            "G6" => {
                                if child_telegram == "100" {
                                    child_telegram1.GasConsumption.push(parsed_value * 100.0);
                                } else if child_telegram == "010" {
                                    child_telegram2.GasConsumption.push(parsed_value * 100.0);
                                } else if child_telegram == "001" {
                                    child_telegram3.GasConsumption.push(parsed_value * 100.0);
                                } else {
                                    telegram.GasConsumption.push(parsed_value * 100.0);
                                }
                            }
                            _ => {
                                println!("Random Non-GasConsumption Value: {:?}", parsed_value);
                                std::process::exit(42); // Exit the program when needed
                                // Handle the error as needed
                            }
                        }
                    }
                    else {
                        println!("Missing Parsed GasConsumption Value: {}", payload);
                        std::process::exit(42); // Exit the program when needed
                    }
                }
                _ => {
                    println!("Failed to parse GasConsumption: {}", payload);
                    std::process::exit(42); // Exit the program when needed
                }
            }
            Ok(Keys::WaterConsumption) => match payload.trim_end_matches("*L").parse::<u64>() {
                Ok(parsed_value) => telegram.WaterConsumption.push(parsed_value),
                Err(_) => {
                    println!("Failed to parse WaterConsumption: {}", payload);
                    std::process::exit(42);
                }
            },
            Ok(Keys::VoltageP1) => match payload.trim_end_matches("*V").parse::<f64>() {
                Ok(parsed_value) => telegram.VoltageP1.push(parsed_value),
                Err(_) => {
                    println!("Failed to parse VoltageP1: {}", payload);
                    std::process::exit(42);
                }
            },
            Ok(Keys::VoltageP2) => match payload.trim_end_matches("*V").parse::<f64>() {
                Ok(parsed_value) => telegram.VoltageP2.push(parsed_value),
                Err(_) => {
                    println!("Failed to parse VoltageP2: {}", payload);
                    std::process::exit(42);
                }
            },
            Ok(Keys::VoltageP3) => match payload.trim_end_matches("*V").parse::<f64>() {
                Ok(parsed_value) => telegram.VoltageP3.push(parsed_value),
                Err(_) => {
                    println!("Failed to parse VoltageP3: {}", payload);
                    std::process::exit(42);
                }
            },
            Ok(Keys::CurrentP1) => match payload.trim_end_matches("*A").parse::<f64>() {
                Ok(parsed_value) => telegram.CurrentP1.push(parsed_value),
                Err(_) => {
                    println!("Failed to parse CurrentP1: {}", payload);
                    std::process::exit(42);
                }
            },
            Ok(Keys::CurrentP2) => match payload.trim_end_matches("*A").parse::<f64>() {
                Ok(parsed_value) => telegram.CurrentP2.push(parsed_value),
                Err(_) => {
                    println!("Failed to parse CurrentP2: {}", payload);
                    std::process::exit(42);
                }
            },
            Ok(Keys::CurrentP3) => match payload.trim_end_matches("*A").parse::<f64>() {
                Ok(parsed_value) => telegram.CurrentP3.push(parsed_value),
                Err(_) => {
                    println!("Failed to parse CurrentP3: {}", payload);
                    std::process::exit(42);
                }
            },
            Ok(Keys::PowerP1) => match payload.trim_end_matches("*kW").parse::<f64>() {
                Ok(parsed_value) => telegram.PowerP1.push(parsed_value),
                Err(_) => {
                    println!("Failed to parse PowerP1: {}", payload);
                    std::process::exit(42);
                }
            },
            Ok(Keys::PowerP2) => match payload.trim_end_matches("*kW").parse::<f64>() {
                Ok(parsed_value) => telegram.PowerP2.push(parsed_value),
                Err(_) => {
                    println!("Failed to parse PowerP2: {}", payload);
                    std::process::exit(42);
                }
            },
            Ok(Keys::PowerP3) => match payload.trim_end_matches("*kW").parse::<f64>() {
                Ok(parsed_value) => telegram.PowerP3.push(parsed_value),
                Err(_) => {
                    println!("Failed to parse PowerP3: {}", payload);
                    std::process::exit(42);
                }
            },
            Ok(Keys::EnergyConsumption) => match payload.trim_end_matches("*kWh").parse::<f64>() {
                Ok(parsed_value) => telegram.EnergyConsumption.push(parsed_value),
                Err(_) => {
                    println!("Failed to parse EnergyConsumption: {}", payload);
                    std::process::exit(42);
                }
            },
            Ok(Keys::EnergyProduction) => match payload.trim_end_matches("*kWh").parse::<f64>() {
                Ok(parsed_value) => telegram.EnergyProduction.push(parsed_value),
                Err(_) => {
                    println!("Failed to parse EnergyProduction: {}", payload);
                    std::process::exit(42);
                }
            },
            Ok(Keys::End) => telegram.Telegram_End.push(payload.to_string()), // println!("End - {:?}", payload),
            Ok(Keys::LineBreak) => {
                // Do nothing
            },
            Err(e) => {
                println!("Invalid telegram. Exiting with exit code 42. {e}");
                std::process::exit(42); // Exit with code 42 for invalid telegrams
            }
            Ok(Keys::ChildTelegram1) => {
                // println!("{:?}", payload);
                child_telegram1.Telegram_Start.push(payload.to_string());
                child_telegram = "100";
                // println!("{:#?}",child_telegram1);
            },
            Ok(Keys::ChildTelegram2) => {
                // println!("{:?}", payload);
                child_telegram2.Telegram_Start.push(payload.to_string());
                child_telegram = "010";
                // println!("{:#?}",child_telegram2);
            },
            Ok(Keys::ChildTelegram3) => {
                // println!("{:?}", payload);
                child_telegram3.Telegram_Start.push(payload.to_string());
                child_telegram = "001";
                // println!("{:#?}",child_telegram3);
            },
            Ok(Keys::EndChildTelegram1) => {
                child_telegram1.Telegram_End.push(payload.to_string());
                child_telegram = "000";
                // println!("{:#?}",child_telegram1);
            },            
            Ok(Keys::EndChildTelegram2) => {
                child_telegram2.Telegram_End.push(payload.to_string());
                child_telegram = "000";
                // println!("{:#?}",child_telegram2);
            },                        
            Ok(Keys::EndChildTelegram3) => {
                child_telegram3.Telegram_End.push(payload.to_string());
                child_telegram = "000";
                // println!("{:#?}",child_telegram3);
            },                          
        };
    }
    // println!("{:#?}",telegram);
    let parsed_dates: Vec<Option<(u16, u8, u8, u8, u8, u8, bool)>> = telegram.Telegram_Date.iter().map(|date| parse_datetime(date)).collect();

    // Iterate through the parsed dates and print them
    for parsed_date in parsed_dates {
        match parsed_date {
            Some((year, month, day, hour, minute, seconds, dst)) => {
                // println!(
                //     "YY:{}, MM:{}, DD:{}, HH:{}, MM:{}, SS:{}, DST:{}",
                //     year, month, day, hour, minute, seconds, dst
                // );
                let time_stamp = date_to_timestamp(year, month, day, hour, minute, seconds, dst).unwrap_or_default();
                // println!("{:?}",timestamp);
                telegram.TimeStamp.push(time_stamp);
                let result = tudelft_dsmr_output_generator::date_to_timestamp(year, month, day, hour, minute, seconds, dst);
            }
            None => {
                println!("Failed to parse date and time.");
                std::process::exit(42);
            }
        }
    }
    // println!("{:#?}",child_telegram1);

    // Error Handling with Exit 42
    if telegram.Telegram_Date.len() > 0 {
        println!("Telegram Date exists. Number of Dates - {:?}", telegram.Telegram_Date.len());
    } else {
        println!("Invalid Telegram with No Single Date");
        std::process::exit(42);
    }
    if telegram.EventLogDate.len() == telegram.EventLogMessage.len() && telegram.EventLogDate.len() == telegram.EventLogSeverity.len() {
        println!("Equal Number of Values for Event Log Date, Messages and Severity - {:?}", telegram.EventLogDate.len());
    } else {
        println!("The number of values for Event Log Date, Messages and Severity do not match!");
        std::process::exit(42);
    }
    if telegram.InformationType.contains(&"W".to_string()) && telegram.WaterConsumption.len() > 0 {
        println!("There is a Water Telegram and Number of Water Consumption is {:?} > 0", telegram.InformationType.len());
    } else {
        println!("The telegram is for {:?} Data but No {:?} Data can be found.", telegram.InformationType, telegram.InformationType);
        std::process::exit(42);
    }
    if telegram.InformationType.contains(&"E".to_string()) && telegram.VoltageP1.len() > 0 {
        println!("There is Electricity Data. Further Checks are Needed. Currently, Number of VoltageP1 Data - {:?}", telegram.VoltageP1.len());
    } else {
        println!("The telegram is for {:?} Data but No {:?} Data can be found.", telegram.InformationType, telegram.InformationType);
        std::process::exit(42);
    }
    if telegram.InformationType.contains(&"G".to_string()) && child_telegram1.GasConsumption.len() > 0 {
        println!("There is Gas Data. Further Checks are Needed. Currently, Number of Gas Data - {:?}", telegram.InformationType.len());
    } else {
        println!("The telegram is for {:?} but No {:?} Data can be found.", telegram.InformationType, telegram.InformationType);
        std::process::exit(42);
    }
    if telegram.VoltageP1.len() == telegram.VoltageP2.len() && telegram.VoltageP1.len() == telegram.VoltageP3.len() {
        println!("Equal Number of Values for Each Phase of Voltage - {:?}", telegram.VoltageP1.len());
    } else {
        println!("The number of values for each phase of Voltage do not match!");
        std::process::exit(42);
    }
    if telegram.CurrentP1.len() == telegram.CurrentP2.len() && telegram.CurrentP1.len() == telegram.CurrentP3.len() {
        println!("Equal Number of Values for Each Phase of Current - {:?}", telegram.CurrentP1.len());
    } else {
        println!("The number of values for each phase of Current do not match!");
        std::process::exit(42);
    }
    if telegram.PowerP1.len() == telegram.PowerP2.len() && telegram.PowerP1.len() == telegram.PowerP3.len() {
        println!("Equal Number of Values for Each Phase of Power - {:?}", telegram.CurrentP1.len());
    } else {
        println!("The number of values for each phase of Power do not match!");
        std::process::exit(42);
    }
    if telegram.VoltageP1.len() == telegram.CurrentP1.len() && telegram.VoltageP1.len() == telegram.PowerP1.len() {
        println!("Equal Number of Values for Each Phase of Voltage, Current, and Power - {:?}", telegram.VoltageP1.len());
    } else {
        println!("The number of values for each phase of Voltage, Current and Power do not match!");
        std::process::exit(42);
    }
    if telegram.EnergyProduction.len() == telegram.EnergyConsumption.len() {
        println!("Equal Number of Values for Energy Production and Energy Consumption - {:?}", telegram.EnergyConsumption.len());
    } else {
        println!("The number of values for Energy Production and Consumption do not match!");
        std::process::exit(42);
    }

    telegram.ChildTelegram1 = child_telegram1;
    telegram.ChildTelegram2 = child_telegram2;
    telegram.ChildTelegram3 = child_telegram3;
    telegram.GasConsumption.extend(telegram.ChildTelegram1.GasConsumption.iter().cloned());
    telegram.GasConsumption.extend(telegram.ChildTelegram2.GasConsumption.iter().cloned());
    telegram.GasConsumption.extend(telegram.ChildTelegram3.GasConsumption.iter().cloned());    
    println!("{:#?}",&telegram);
    // for Telegram_Date in &telegram.Telegram_Date { println!("{}", Telegram_Date); }
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
    let voltage_values: Vec<VoltageData> = vector_voltage(parsed.VoltageP1.clone(), parsed.VoltageP2.clone(), parsed.VoltageP3.clone(), parsed.TimeStamp.clone());
    result.add_graph(create_voltage_over_time_graph(voltage_values))?;
    
    let current_values: Vec<CurrentData> = vector_current(parsed.CurrentP1.clone(), parsed.CurrentP2.clone(), parsed.CurrentP3.clone(), parsed.TimeStamp.clone());
    let mut current_graph = CurrentOverTime::new(); // Create an instance of CurrentOverTime
    for data_current in current_values {
        current_graph.add(data_current); // Add each CurrentData instance to the graph by moving it
    }
    let _ = result.add_graph(current_graph);

    let energy_values: Vec<EnergyData> = vector_energy(parsed.EnergyProduction.clone(), parsed.EnergyConsumption.clone(), parsed.TimeStamp.clone());
    let mut energy_graph = EnergyOverTime::new(); // Create an instance of EnergyOverTime
    for data_energy in energy_values {
        energy_graph.add(data_energy); // Add each EnergyData instance to the graph by moving it
    }
    let _ = result.add_graph(energy_graph);

    let water_values: Vec<WaterData> = vector_water(parsed.WaterConsumption.clone(), parsed.TimeStamp.clone());
    let mut water_graph = WaterOverTime::new(); // Create an instance of WaterOverTime
    for data_water in water_values {
        water_graph.add(data_water); // Add each WaterData instance to the graph by moving it
    }
    let _ = result.add_graph(water_graph);

    let gas_values: Vec<GasData> = vector_gas(parsed.GasConsumption.clone(), parsed.TimeStamp.clone());
    let mut gas_graph = GasOverTime::new(); // Create an instance of GasOverTime
    for data_gas in gas_values {
        gas_graph.add(data_gas); // Add each GasData instance to the graph by moving it
    }
    let _ = result.add_graph(gas_graph);

    result.generate().expect("error generating graphs");
    Ok(())
}
