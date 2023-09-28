use error::MainError;
use std::io::{Read};
use std::process::{id, exit};
use tudelft_dsmr_output_generator::voltage_over_time::{
    create_voltage_over_time_graph, VoltageData,
};
use tudelft_dsmr_output_generator::{Graphs, date_to_timestamp};
/// Contains `MainError`, and code to convert `PlotError` and `io::Error` into a `MainError`
mod error;

enum Versions {
    V10,
    V12,
}
enum Keys {
    Start, // 1.1.n
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
}
enum Extensions {
    Gas,
    Recursive,
}
struct Header {
    version: Option<Versions>, // None if no header found yet.
    extensions: Vec<Extensions> 
}
#[derive(Debug)]
struct Telegram{
    Telegram_Version: String, // 0.0
    Telegram_Start: Vec<String>, // 1.1
    Telegram_Date: Vec<String>, // 2.1
    EventLogSeverity: Vec<String>, // 3.1.n
    EventLogMessage: Vec<String>, // 3.2.n
    EventLogDate: Vec<String>, // 3.3.n
    InformationType: String, // 4.1
    GasModel: String, // 5.1
    GasConsumption: f64, // 5.2
    WaterConsumption: u64, // 6.1
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
    DateCode: Vec<i64>, // 2.1.n
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
fn telegram_ver(telegram_version: &str) -> Result<Versions, &'static str> {
    match telegram_version {
        "10" => Ok(Versions::V10),
        "12" => Ok(Versions::V12),
        _ => Err("Neither Version 12 or 10"),
    }
}
fn version_key(version_key: &str) -> Result<Keys, String> {
    match version_key {
        key if key.starts_with("1.1.") => Ok(Keys::Start),
        key if key.starts_with("2.1") => Ok(Keys::Date),
        key if key.starts_with("1.2.") => Ok(Keys::End),
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
        other => Err(format!("Authentication Failed: {other}")),
        // _ => Err("Authentication Failed"),
    }
}
fn hex_string_to_string(hex_str: &str) -> String {
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
fn parse(input: &str) -> Result<(), MainError> {
    // cd User/Y1S1-SoftwareFundamentals/2023-09-12_Proj-Individual/btee
    // Note that you can use this function:
    // tudelft_dsmr_output_generator::date_to_timestamp(year, month, day, hour, minute, seconds, dst)
    // let l = lines.len(); // print!("{}",l); // print!("{}",input);
    // let Test = _lines[1].split('#').next().unwrap().trim();
    // println!("{:?}", _lines); // println!("{:?}", Test);
    
    let mut telegram = Telegram {
        Telegram_Version: String::new(),
        Telegram_Start: Vec::new(),
        Telegram_Date: Vec::new(),
        EventLogSeverity: Vec::new(),
        EventLogMessage: Vec::new(),
        EventLogDate: Vec::new(),
        InformationType: String::new(),
        GasModel: String::new(),
        GasConsumption: 0.0,
        WaterConsumption: 0,
        VoltageP1: Vec::new(),
        VoltageP2: Vec::new(),
        VoltageP3: Vec::new(),
        CurrentP1: Vec::new(),
        CurrentP2: Vec::new(),
        CurrentP3: Vec::new(),
        PowerP1: Vec::new(),
        PowerP2: Vec::new(),
        PowerP3: Vec::new(),
        EnergyConsumption: Vec::new(),
        EnergyProduction: Vec::new(),
        Telegram_End: Vec::new(),
        DateCode: Vec::new(),
    };
    let telegram_version = &input[2..4].to_string();
    match telegram_ver(&telegram_version) {
        Ok(Versions::V10) => telegram.Telegram_Version = telegram_version.to_string(), // println!("Version - {:?}", telegram_version),
        Ok(Versions::V12) => telegram.Telegram_Version = telegram_version.to_string(), // println!("Version - {:?}", telegram_version),
        _ => println!("Neither Version 12 nor 10"),
    };

    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        // println!("{}", line);
        let (telegram_id, payload) = process_lines(line);
        // println!("Telegram ID: {}, Payload: {}", telegram_id, payload);
        let telegram_id_clone = telegram_id.to_string(); // Clone telegram_id
        match version_key(&telegram_id_clone) {
            Ok(Keys::Start) => telegram.Telegram_Start.push(payload.to_string()), // println!("Start - {:?}", payload),
            Ok(Keys::Date) => telegram.Telegram_Date.push(payload.to_string()), // println!("Date - {:?}", payload),
            Ok(Keys::EventLogSeverity) => telegram.EventLogSeverity.push(payload.to_string()), // Handle as a vector
            Ok(Keys::EventLogMessage) => telegram.EventLogMessage.push(hex_string_to_string(&payload)), // .to_string(), // println!("Date - {:?}", payload),
            Ok(Keys::EventLogDate) => telegram.EventLogDate.push(payload.to_string()), // println!("Date - {:?}", payload),
            Ok(Keys::InformationType) => telegram.InformationType = payload.to_string(), // println!("Date - {:?}", payload),
            Ok(Keys::GasModel) => telegram.GasModel = payload.to_string(), // println!("Date - {:?}", payload),
            Ok(Keys::GasConsumption) => telegram.GasConsumption = payload.trim_end_matches("*m3").parse().unwrap(), // Parse as u64
            Ok(Keys::WaterConsumption) => telegram.WaterConsumption = payload.trim_end_matches("*L").parse().unwrap(), // Parse as u64
            Ok(Keys::VoltageP1) => match payload.trim_end_matches("*V").parse::<f64>() {
                Ok(parsed_value) => telegram.VoltageP1.push(parsed_value),
                Err(_) => println!("Failed to parse VoltageP1: {}", payload),
            },
            Ok(Keys::VoltageP2) => match payload.trim_end_matches("*V").parse::<f64>() {
                Ok(parsed_value) => telegram.VoltageP2.push(parsed_value),
                Err(_) => println!("Failed to parse VoltageP2: {}", payload),
            },
            Ok(Keys::VoltageP3) => match payload.trim_end_matches("*V").parse::<f64>() {
                Ok(parsed_value) => telegram.VoltageP3.push(parsed_value),
                Err(_) => println!("Failed to parse VoltageP3: {}", payload),
            },
            Ok(Keys::CurrentP1) => match payload.trim_end_matches("*A").parse::<f64>() {
                Ok(parsed_value) => telegram.CurrentP1.push(parsed_value),
                Err(_) => println!("Failed to parse CurrentP1: {}", payload),
            },
            Ok(Keys::CurrentP2) => match payload.trim_end_matches("*A").parse::<f64>() {
                Ok(parsed_value) => telegram.CurrentP2.push(parsed_value),
                Err(_) => println!("Failed to parse CurrentP2: {}", payload),
            },
            Ok(Keys::CurrentP3) => match payload.trim_end_matches("*A").parse::<f64>() {
                Ok(parsed_value) => telegram.CurrentP3.push(parsed_value),
                Err(_) => println!("Failed to parse CurrentP3: {}", payload),
            },
            Ok(Keys::PowerP1) => match payload.trim_end_matches("*kW").parse::<f64>() {
                Ok(parsed_value) => telegram.PowerP1.push(parsed_value),
                Err(_) => println!("Failed to parse PowerP1: {}", payload),
            },
            Ok(Keys::PowerP2) => match payload.trim_end_matches("*kW").parse::<f64>() {
                Ok(parsed_value) => telegram.PowerP2.push(parsed_value),
                Err(_) => println!("Failed to parse PowerP2: {}", payload),
            },
            Ok(Keys::PowerP3) => match payload.trim_end_matches("*kW").parse::<f64>() {
                Ok(parsed_value) => telegram.PowerP3.push(parsed_value),
                Err(_) => println!("Failed to parse PowerP3: {}", payload),
            },
            Ok(Keys::EnergyConsumption) => match payload.trim_end_matches("*kWh").parse::<f64>() {
                Ok(parsed_value) => telegram.EnergyConsumption.push(parsed_value),
                Err(_) => println!("Failed to parse EnergyConsumption: {}", payload),
            },
            Ok(Keys::EnergyProduction) => match payload.trim_end_matches("*kWh").parse::<f64>() {
                Ok(parsed_value) => telegram.EnergyProduction.push(parsed_value),
                Err(_) => println!("Failed to parse EnergyProduction: {}", payload),
            },
            Ok(Keys::End) => telegram.Telegram_End.push(payload.to_string()), // println!("End - {:?}", payload),
            Ok(Keys::LineBreak) => {
                // Do nothing
            },
            Err(e) => {
                println!("Invalid telegram. Exiting with exit code 42. {e}");
                std::process::exit(42); // Exit with code 42 for invalid telegrams
            }
            //println!("{:?}", payload),
        };
    }
    // println!("{:#?}",telegram);
    let parsed_dates: Vec<Option<(u16, u8, u8, u8, u8, u8, bool)>> = telegram
    .Telegram_Date
    .iter()
    .map(|date| parse_datetime(date))
    .collect();

    // Iterate through the parsed dates and print them
    for parsed_date in parsed_dates {
        match parsed_date {
            Some((year, month, day, hour, minute, seconds, dst)) => {
                // println!(
                //     "YY:{}, MM:{}, DD:{}, HH:{}, MM:{}, SS:{}, DST:{}",
                //     year, month, day, hour, minute, seconds, dst
                // );
                let datecode = date_to_timestamp(year, month, day, hour, minute, seconds, dst).unwrap_or_default();
                // println!("{:?}",datecode);
                telegram.DateCode.push(datecode);
                let result = tudelft_dsmr_output_generator::date_to_timestamp(year, month, day, hour, minute, seconds, dst);
            }
            None => {
                println!("Failed to parse date and time.");
            }
        }
    }
    println!("{:#?}",telegram);
    // for Telegram_Date in &telegram.Telegram_Date {
    //     println!("{}", Telegram_Date);
    // }
    Ok(())
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
    let _parsed = parse(&input)?;
    let mut result = Graphs::new()?;

    // let result = tudelft_dsmr_output_generator::date_to_timestamp(year, month, day, hour, minute, seconds, dst)

    result.add_graph(create_voltage_over_time_graph(vec![
        VoltageData {
            phase_1: 100.0,
            phase_2: 200.0,
            phase_3: 300.0,     
            timestamp: 100,
        },
        VoltageData {
            phase_1: 200.0,
            phase_2: 300.0,
            phase_3: 250.0,
            timestamp: 10000,
        },
    ]))?;
    result.generate().expect("error generating graphs");
    Ok(())
}