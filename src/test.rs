#[cfg(test)]
use crate::parse_datetime;
use super::*;
#[test]
fn test_telegram_ver_pass() {
    let input = "12";
    let result = telegram_ver(input);
    assert_eq!(result, Ok(Versions::V12));
}

#[test]
fn test_telegram_ver_fail() {
    let input = "11";
    let result = telegram_ver(input);
    assert_ne!(result, Ok(Versions::V12));
}

#[test]
fn test_version_extension_pass() {
    let input = "gr";
    let result = version_ext(input);
    assert_eq!(result, Ok(Extensions::GasRecursive));
}

#[test]
fn test_version_extension_2() {
    let input = "ab";
    let result = version_ext(input);
    assert_ne!(result, Ok(Extensions::Recursive));
}

#[test]
fn test_process_lines_pass1() {
    let line = "1.2.0#(END)";
    let result = process_lines(line);
    assert_eq!(result, ("1.2.0".to_string(), "END".to_string()));
}

#[test]
fn test_process_lines_fail1() {
    let line = "1.2.0#(END)";
    let result = process_lines(line);
    assert_ne!(result, ("1.2.".to_string(), "ND".to_string()));
}

#[test]
fn test_process_lines_pass2() {
    let line = " ";
    let result = process_lines(line);
    assert_eq!(result, ("LineBreak".to_string(), "".to_string()));
}

#[test]
fn test_process_lines_fail2() {
    let line = " ";
    let result = process_lines(line);
    assert_ne!(result, ("Line Space".to_string(), "".to_string()));
}

#[test]
fn test_version_key_pass() {
    let line = "1.1.0";
    let result = version_key(line);
    assert_eq!(result, (Ok(Keys::Start)));
}

#[test]
fn test_version_key_fail() {
    let line = "1.1.1";
    let result = version_key(line);
    assert_ne!(result, (Ok(Keys::Start)));
}

#[test]
fn test_hex_string_pass() {
    let line = "506f776572204661696c757265";
    let result = hex_string(line);
    assert_eq!(result, "Power Failure");
}

#[test] //
fn test_hex_string_fail() {
    let line = "506f776572204661696c757265";
    let result = hex_string(line);
    assert_ne!(result, "Power Failure.");
}

#[test]
fn test_parse_datetime_jan_pass() {
    let date_message = "22-Jan-22 12:34:56 (S)";
    let result = parse_datetime(date_message);
    assert_eq!(result, Some((2022, 1, 22, 12, 34, 56, true)));
}
#[test]
fn test_parse_datetime_apr_pass() {
    let date_message = "22-Apr-22 12:34:56 (S)";
    let result = parse_datetime(date_message);
    assert_eq!(result, Some((2022, 4, 22, 12, 34, 56, true)));
}

#[test]
fn test_parse_datetime_fail() {
    let date_message = "22-Jan-22 12:34:56 (S)";
    let result = parse_datetime(date_message);
    assert_ne!(result, Some((22, 1, 22, 12, 34, 56, true)));
}

#[test]
fn test_vector_water_pass() {
    let consumed = vec![0];
    let time = vec![1072914282];
    let result = vector_water(consumed, time);

    // Convert vectors to strings for comparison
    let expected_str = format!("{:?}", vec![WaterData { water_delta: 0, timestamp: 1072914282 }]);
    let result_str = format!("{:?}", result);

    assert_eq!(result_str, expected_str);
}

#[test]
fn test_vector_water_fail() {
    let consumed = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    let time = vec![1072914282, 1072914283, 1072914284, 1072914285, 1072914286, 1072914287, 1072914288, 1072914289, 1072914290, 1072914291, 1072914292, 1072914293, 1072914294, 1072914295];
    let result = vector_water(consumed, time);

    // Convert vectors to strings for comparison
    let expected_str = format!("{:?}", vec![WaterData { water_delta: 0, timestamp: 1072914282 }]);
    let result_str = format!("{:?}", result);

    assert_ne!(result_str, expected_str);
}

#[test]
fn test_vector_energy_pass() {
    let produced = vec![0.0];
    let consumed = vec![0.0];
    let time = vec![1072914282];
    let result = vector_energy(produced, consumed, time);

    // Convert vectors to strings for comparison
    let expected_str = format!("{:?}", vec![EnergyData { produced: 0.0, consumed: 0.0, timestamp: 1072914282 }]);
    let result_str = format!("{:?}", result);

    assert_eq!(result_str, expected_str);
}

#[test]
fn test_vector_energy_fail() {
    let produced = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0];
    let consumed = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0];
    let time = vec![1072914282, 1072914283, 1072914284, 1072914285, 1072914286, 1072914287, 1072914288, 1072914289, 1072914290, 1072914291, 1072914292, 1072914293, 1072914294, 1072914295];
    let result = vector_energy(produced, consumed, time);

    // Convert vectors to strings for comparison
    let expected_str = format!("{:?}", vec![EnergyData { produced: 0.0, consumed: 0.0, timestamp: 1072914282 }]);
    let result_str = format!("{:?}", result);

    assert_ne!(result_str, expected_str);
}

#[test]
fn test_vector_gas_pass() {
    let consumed = vec![0.0];
    let time = vec![1072914282];
    let result = vector_gas(consumed, time);

    // Convert vectors to strings for comparison
    let expected_str = format!("{:?}", vec![GasData { gas_delta: 0.0, timestamp: 1072914282 }]);
    let result_str = format!("{:?}", result);

    assert_eq!(result_str, expected_str);
}

#[test]
fn test_vector_gas_fail() {
    let consumed = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0];
    let time = vec![1072914282, 1072914283, 1072914284, 1072914285, 1072914286, 1072914287, 1072914288, 1072914289, 1072914290, 1072914291, 1072914292, 1072914293, 1072914294, 1072914295];
    let result = vector_gas(consumed, time);

    // Convert vectors to strings for comparison
    let expected_str = format!("{:?}", vec![GasData { gas_delta: 0.0, timestamp: 1072914282 }]);
    let result_str = format!("{:?}", result);

    assert_ne!(result_str, expected_str);
}
#[test]
fn test_handle_voltage_p1_pass() {
    // Arrange
    let mut telegram = Telegram::default();
    let payload = "123.45*V";
    let key = "voltage_p1";

    // Act
    handle_voltage(payload, &mut telegram, key);

    // Assert
    assert_eq!(telegram.voltage_p1, vec![123.45]);
    assert!(telegram.voltage_p2.is_empty());
    assert!(telegram.voltage_p3.is_empty());
}
#[test]
fn test_handle_voltage_p2_pass() {
    // Arrange
    let mut telegram = Telegram::default();
    let payload = "123.45*V";
    let key = "voltage_p2";

    // Act
    handle_voltage(payload, &mut telegram, key);

    // Assert
    assert_eq!(telegram.voltage_p2, vec![123.45]);
    assert!(telegram.voltage_p1.is_empty());
    assert!(telegram.voltage_p3.is_empty());
}
#[test]
fn test_handle_voltage_p3_pass() {
    // Arrange
    let mut telegram = Telegram::default();
    let payload = "123.45*V";
    let key = "voltage_p3";

    // Act
    handle_voltage(payload, &mut telegram, key);

    // Assert
    assert_eq!(telegram.voltage_p3, vec![123.45]);
    assert!(telegram.voltage_p1.is_empty());
    assert!(telegram.voltage_p2.is_empty());
}
#[test]
fn test_handle_current_p1_pass() {
    // Arrange
    let mut telegram = Telegram::default();
    let payload = "123.45*A";
    let key = "current_p1";

    // Act
    handle_current(payload, &mut telegram, key);

    // Assert
    assert_eq!(telegram.current_p1, vec![123.45]);
    assert!(telegram.current_p2.is_empty());
    assert!(telegram.current_p3.is_empty());
}
#[test]
fn test_handle_current_p2_pass() {
    // Arrange
    let mut telegram = Telegram::default();
    let payload = "123.45*A";
    let key = "current_p2";

    // Act
    handle_current(payload, &mut telegram, key);

    // Assert
    assert_eq!(telegram.current_p2, vec![123.45]);
    assert!(telegram.current_p1.is_empty());
    assert!(telegram.current_p3.is_empty());
}
#[test]
fn test_handle_current_p3_pass() {
    // Arrange
    let mut telegram = Telegram::default();
    let payload = "123.45*A";
    let key = "current_p3";

    // Act
    handle_current(payload, &mut telegram, key);

    // Assert
    assert_eq!(telegram.current_p3, vec![123.45]);
    assert!(telegram.current_p1.is_empty());
    assert!(telegram.current_p2.is_empty());
}
#[test]
fn test_handle_power_p1_pass() {
    // Arrange
    let mut telegram = Telegram::default();
    let payload = "123.45*kW";
    let key = "power_p1";

    // Act
    handle_power(payload, &mut telegram, key);

    // Assert
    assert_eq!(telegram.power_p1, vec![123.45]);
    assert!(telegram.power_p2.is_empty());
    assert!(telegram.power_p3.is_empty());
}
#[test]
fn test_handle_power_p2_pass() {
    // Arrange
    let mut telegram = Telegram::default();
    let payload = "123.45*kW";
    let key = "power_p2";

    // Act
    handle_power(payload, &mut telegram, key);

    // Assert
    assert_eq!(telegram.power_p2, vec![123.45]);
    assert!(telegram.power_p1.is_empty());
    assert!(telegram.power_p3.is_empty());
}
#[test]
fn test_handle_power_p3_pass() {
    // Arrange
    let mut telegram = Telegram::default();
    let payload = "123.45*kW";
    let key = "power_p3";

    // Act
    handle_power(payload, &mut telegram, key);

    // Assert
    assert_eq!(telegram.power_p3, vec![123.45]);
    assert!(telegram.power_p1.is_empty());
    assert!(telegram.power_p2.is_empty());
}
#[test]
fn test_handle_energy_consumption() {
    let mut telegram = Telegram::default(); // Assuming you have a default implementation for Telegram

    // Simulate an energy consumption payload
    let payload = "123.45*kWh";
    let key = "energy_consumption";
    handle_energy(payload, &mut telegram, key);

    // Assert that the parsed value is correctly added to the energy_consumption vector
    assert_eq!(telegram.energy_consumption, vec![123.45]);
}

#[test]
fn test_handle_energy_production() {
    let mut telegram = Telegram::default(); // Assuming you have a default implementation for Telegram

    // Simulate an energy production payload
    let payload = "67.89*kWh";
    let key = "energy_production";
    handle_energy(payload, &mut telegram, key);

    // Assert that the parsed value is correctly added to the energy_production vector
    assert_eq!(telegram.energy_production, vec![67.89]);
}
#[test]
fn test_handle_gas_consumption_g4() {
    // Test case 1: last_gas_model is "G4", child_telegram is "100"
    let mut child_telegram1 = child_telegram1::default();
    let mut telegram = Telegram::default();
    handle_gas_consumption(5.0, "G4", &mut child_telegram1, &mut telegram, "100");
    assert_eq!(child_telegram1.gas_consumption, vec![5.0]);
}
#[test]
fn test_handle_gas_consumption_g5() {
    // Test case 1: last_gas_model is "G5", child_telegram is "100"
    let mut child_telegram1 = child_telegram1::default();
    let mut telegram = Telegram::default();
    handle_gas_consumption(5.0, "G5", &mut child_telegram1, &mut telegram, "100");
    assert_eq!(child_telegram1.gas_consumption, vec![50.0]);
}
#[test]
fn test_handle_gas_consumption_g6() {
    // Test case 1: last_gas_model is "G6", child_telegram is "100"
    let mut child_telegram1 = child_telegram1::default();
    let mut telegram = Telegram::default();
    handle_gas_consumption(5.0, "G6", &mut child_telegram1, &mut telegram, "100");
    assert_eq!(child_telegram1.gas_consumption, vec![500.0]);
}
#[test]
fn test_parse_datetime() {
    // Test case with a valid input
    let valid_date = "2023-11-15 12:34:56 (S)";
    assert_ne!(parse_datetime(valid_date), Some((2023, 11, 15, 12, 34, 56, true)));
}
#[test]
fn test_vector_voltage() {
    // Sample input data
    let voltage_p1 = vec![0.0];
    let voltage_p2 = vec![0.0];
    let voltage_p3 = vec![0.0];
    let time_stamp = vec![1234567890];

    // Call the function being tested
    let result = vector_voltage(voltage_p1, voltage_p2, voltage_p3, time_stamp);
    let result_str = format!("{:?}", result);
    let expected_str = format!("{:?}", vec![VoltageData {phase_1: 0.0, phase_2: 0.0, phase_3: 0.0, timestamp: 1234567890}]);

    // Assert that the result is as expected
    assert_eq!(result_str, expected_str);
}
#[test]
fn test_vector_current() {
    // Sample input data
    let current_p1 = vec![0.0];
    let current_p2 = vec![0.0];
    let current_p3 = vec![0.0];
    let time_stamp = vec![1234567890];

    // Call the function being tested
    let result = vector_current(current_p1, current_p2, current_p3, time_stamp);
    let result_str = format!("{:?}", result);
    let expected_str = format!("{:?}", vec![CurrentData {phase_1: 0.0, phase_2: 0.0, phase_3: 0.0, timestamp: 1234567890}]);

    // Assert that the result is as expected
    assert_eq!(result_str, expected_str);
}
#[test]
fn test_handle_water_pass() {
    let payload = "1234*L";
    let mut telegram = Telegram::default();
    handle_water(payload, &mut telegram);
    assert_eq!(telegram.water_consumption, vec![1234]);
}