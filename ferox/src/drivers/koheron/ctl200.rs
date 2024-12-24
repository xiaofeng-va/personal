use core::{
    fmt::{self, Display},
    str::{from_utf8, FromStr},
};

use defmt_or_log::{debug, info};
use embedded_io_async::{Read, Write};
use heapless::String;
use crate::{common::MAX_STRING_SIZE, proto::error::Error};
use crate::proto::Result;

const CRLF: &[u8] = b"\r\n";
const CRLF_PROMPT: &[u8] = b"\r\n>>";

// Trait definition with lifetime parameter
trait FromBytes<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self>
    where
        Self: Sized;
}

// Implementation for &[u8]
impl<'a> FromBytes<'a> for &'a [u8] {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        Ok(bytes)
    }
}

// Implementation for bool
impl<'a> FromBytes<'a> for bool {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        match bytes {
            b"0" => Ok(false),
            b"1" => Ok(true),
            _ => Err(Error::InvalidBoolean),
        }
    }
}

// Implementation for i32
impl<'a> FromBytes<'a> for i32 {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        core::str::from_utf8(bytes)
            .map_err(|_| Error::BytesToUTF8Error)?
            .parse()
            .map_err(|_| Error::ParseIntError)
    }
}

// Implementation for f32
impl<'a> FromBytes<'a> for f32 {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        core::str::from_utf8(bytes)
            .map_err(|_| Error::BytesToUTF8Error)?
            .parse()
            .map_err(|_| Error::ParseFloatError)
    }
}

/// Driver for CTL200 Laser Controller
///
/// See <https://www.koheron.com/support/user-guides/ctl200/>.
pub struct Ctl200<U>
where
    U: Read + Write + 'static,
{
    uart: U,
    buf: [u8; MAX_STRING_SIZE],
    buf_pos: usize,
}

impl<U> Ctl200<U>
where
    U: Read + Write + 'static,
{
    pub fn new(uart: U) -> Self {
        Ctl200 {
            uart,
            buf: [0; MAX_STRING_SIZE],
            buf_pos: 0,
        }
    }

    /// Returns the enabled state of the laser.
    pub async fn laser_en(&mut self) -> Result<bool> {
        let is_on = self.get::<i32>("lason").await? == 1;
        debug!("lason: {}", is_on);
        Ok(is_on)
    }

    /// Sets the enabled state of the laser.
    pub async fn set_laser_en(&mut self, en: bool) -> Result<()> {
        debug!("set lason: {}", en);
        self.set("lason", Value::Bool(en)).await
    }

    /// Returns the laser current in mA.
    #[allow(non_snake_case)]
    pub async fn laser_current_mA(&mut self) -> Result<f32> {
        let i_mA = self.get::<f32>("ilaser").await?;
        debug!("ilaser: {} mA", i_mA);
        Ok(i_mA)
    }

    /// Sets the laser current in mA.
    #[allow(non_snake_case)]
    pub async fn set_laser_current_mA(&mut self, i_mA: f32) -> Result<()> {
        debug!("set ilaser: {} mA", i_mA);
        self.set("ilaser", Value::Float(i_mA)).await
    }

    /// Returns the laser voltage in V.
    #[allow(non_snake_case)]
    pub async fn laser_V(&mut self) -> Result<f32> {
        let volts = self.get::<f32>("vlaser").await?;
        debug!("vlaser: {} V", volts);
        Ok(volts)
    }

    /// Returns the laser turn-on delay in ms.
    pub async fn laser_delay_ms(&mut self) -> Result<f32> {
        let delay_ms = self.get::<f32>("ldelay").await?;
        debug!("ldelay: {} ms", delay_ms);
        Ok(delay_ms)
    }

    /// Sets the laser turn-on delay in ms.
    pub async fn set_laser_delay_ms(&mut self, delay_ms: f32) -> Result<()> {
        debug!("set ldelay: {} ms", delay_ms);
        self.set("ldelay", Value::Float(delay_ms)).await
    }

    /// Returns the laser current limit in mA.
    #[allow(non_snake_case)]
    pub async fn current_limit_mA(&mut self) -> Result<f32> {
        let limit_mA = self.get::<f32>("ilmax").await?;
        debug!("ilmax: {} mA", limit_mA);
        Ok(limit_mA)
    }

    /// Sets the laser current limit in mA.
    #[allow(non_snake_case)]
    pub async fn set_current_limit_mA(&mut self, limit_mA: f32) -> Result<()> {
        debug!("set ilmax: {} mA", limit_mA);
        self.set("ilmax", Value::Float(limit_mA)).await
    }

    /// Returns the enabled state of the laser interlock.
    pub async fn interlock_en(&mut self) -> Result<bool> {
        let is_on = self.get::<i32>("lckon").await? == 1;
        debug!("lckon: {}", is_on);
        Ok(is_on)
    }

    /// Sets the enabled state of the laser interlock.
    pub async fn set_interlock_en(&mut self, en: bool) -> Result<()> {
        debug!("set lckon: {}", en);
        self.set("lckon", Value::Bool(en)).await
    }

    /// Returns the laser current modulation gain in mA/V.
    #[allow(non_snake_case)]
    pub async fn laser_current_mod_gain_mA_V(&mut self) -> Result<f32> {
        let gain = self.get::<f32>("lmodgain").await?;
        debug!("lmodgain: {} mA/V", gain);
        Ok(gain)
    }

    /// Sets the laser current modulation gain in mA/V.
    #[allow(non_snake_case)]
    pub async fn set_laser_current_mod_gain_mA_V(&mut self, gain_mA_V: f32) -> Result<()> {
        debug!("set lmodgain: {} mA/V", gain_mA_V);
        self.set("lmodgain", Value::Float(gain_mA_V)).await
    }

    /// Returns the enabled state of the TEC.
    pub async fn tec_en(&mut self) -> Result<bool> {
        let is_on = self.get::<i32>("tecon").await? == 1;
        debug!("tecon: {}", is_on);
        Ok(is_on)
    }

    /// Sets the enabled state of the TEC.
    pub async fn set_tec_en(&mut self, en: bool) -> Result<()> {
        debug!("set tecon: {}", en);
        self.set("tecon", Value::Bool(en)).await
    }

    /// Returns the enabled state of the temperature protection.
    pub async fn temp_prot_en(&mut self) -> Result<bool> {
        let is_on = self.get::<i32>("tprot").await? == 1;
        debug!("tprot: {}", is_on);
        Ok(is_on)
    }

    /// Sets the enabled state of the temperature protection.
    pub async fn set_temp_prot_en(&mut self, en: bool) -> Result<()> {
        debug!("set tprot: {}", en);
        self.set("tprot", Value::Bool(en)).await
    }

    /// Returns the thermistor setpoint in Ohms.
    #[allow(non_snake_case)]
    pub async fn temp_set_Ohm(&mut self) -> Result<f32> {
        let setpoint_ohms = self.get::<f32>("rtset").await?;
        debug!("rtset: {} Ohms", setpoint_ohms);
        Ok(setpoint_ohms)
    }

    /// Sets the thermistor setpoint in Ohms.
    #[allow(non_snake_case)]
    pub async fn set_temp_set_Ohm(&mut self, setpoint_Ohms: f32) -> Result<()> {
        debug!("set rtset: {} Ohms", setpoint_Ohms);
        self.set("rtset", Value::Float(setpoint_Ohms)).await
    }

    /// Returns the actual thermistor reading in Ohms.
    #[allow(non_snake_case)]
    pub async fn temp_act_Ohm(&mut self) -> Result<f32> {
        let curr_val = self.get::<f32>("rtact").await?;
        debug!("rtact: {} Ohms", curr_val);
        Ok(curr_val)
    }

    /// Returns the TEC current in A.
    #[allow(non_snake_case)]
    pub async fn tec_current_A(&mut self) -> Result<f32> {
        let curr_val = self.get::<f32>("itec").await?;
        debug!("itec: {} A", curr_val);
        Ok(curr_val)
    }

    /// Returns the TEC voltage in V.
    #[allow(non_snake_case)]
    pub async fn tec_voltage_V(&mut self) -> Result<f32> {
        let curr_val = self.get::<f32>("vtec").await?;
        debug!("vtec: {} V", curr_val);
        Ok(curr_val)
    }

    /// Returns the proportional gain of the temperature controller.
    pub async fn prop_gain(&mut self) -> Result<f32> {
        let curr_val = self.get::<f32>("pgain").await?;
        debug!("pgain: {}", curr_val);
        Ok(curr_val)
    }

    /// Sets the proportional gain of the temperature controller.
    pub async fn set_prop_gain(&mut self, gain: f32) -> Result<()> {
        debug!("set pgain: {}", gain);
        self.set("pgain", Value::Float(gain)).await
    }

    /// Returns the integral gain of the temperature controller.
    pub async fn int_gain(&mut self) -> Result<f32> {
        let curr_val = self.get::<f32>("igain").await?;
        debug!("igain: {}", curr_val);
        Ok(curr_val)
    }

    /// Sets the integral gain of the temperature controller.
    pub async fn set_int_gain(&mut self, gain: f32) -> Result<()> {
        debug!("set igain: {}", gain);
        self.set("igain", Value::Float(gain)).await
    }

    /// Returns the differential gain of the temperature controller.
    pub async fn diff_gain(&mut self) -> Result<f32> {
        let curr_val = self.get::<f32>("dgain").await?;
        debug!("dgain: {}", curr_val);
        Ok(curr_val)
    }

    /// Sets the differential gain of the temperature controller.
    pub async fn set_diff_gain(&mut self, gain: f32) -> Result<()> {
        debug!("set dgain: {}", gain);
        self.set("dgain", Value::Float(gain)).await
    }

    /// Returns the lower temperature limit in Ohms.
    #[allow(non_snake_case)]
    pub async fn temp_min_Ohm(&mut self) -> Result<f32> {
        let value = self.get::<f32>("rtmin").await?;
        debug!("rtmin: {} Ohms", value);
        Ok(value)
    }

    /// Sets the lower temperature limit in Ohms.
    #[allow(non_snake_case)]
    pub async fn set_temp_min_Ohm(&mut self, min: f32) -> Result<()> {
        debug!("set rtmin: {} Ohms", min);
        self.set("rtmin", Value::Float(min)).await
    }

    /// Returns the upper temperature limit in Ohms.
    #[allow(non_snake_case)]
    pub async fn temp_max_Ohm(&mut self) -> Result<f32> {
        let value = self.get::<f32>("rtmax").await?;
        debug!("rtmax: {} Ohms", value);
        Ok(value)
    }

    /// Sets the upper temperature limit in Ohms.
    #[allow(non_snake_case)]
    pub async fn set_temp_max_Ohm(&mut self, max: f32) -> Result<()> {
        debug!("set rtmax: {} Ohms", max);
        self.set("rtmax", Value::Float(max)).await
    }

    /// Returns the minimum TEC voltage in V.
    #[allow(non_snake_case)]
    pub async fn tec_min_V(&mut self) -> Result<f32> {
        let val = self.get::<f32>("vtmin").await?;
        debug!("vtmin: {} V", val);
        Ok(val)
    }

    /// Sets the minimum TEC voltage in V.
    #[allow(non_snake_case)]
    pub async fn set_tec_min_V(&mut self, volts: f32) -> Result<()> {
        debug!("set vtmin: {} V", volts);
        self.set("vtmin", Value::Float(volts)).await
    }

    /// Returns the maximum TEC voltage in V.
    #[allow(non_snake_case)]
    pub async fn tec_max_V(&mut self) -> Result<f32> {
        let val = self.get::<f32>("vtmax").await?;
        debug!("vtmax: {} V", val);
        Ok(val)
    }

    /// Sets the maximum TEC voltage in V.
    #[allow(non_snake_case)]
    pub async fn set_tec_max_V(&mut self, volts: f32) -> Result<()> {
        debug!("set vtmax: {} V", volts);
        self.set("vtmax", Value::Float(volts)).await
    }

    /// Returns the temperature modulation gain in Ohms/V.
    #[allow(non_snake_case)]
    pub async fn temp_mod_gain_Ohm_V(&mut self) -> Result<f32> {
        let gain = self.get::<f32>("tmodgain").await?;
        debug!("tmodgain: {} Ohms/V", gain);
        Ok(gain)
    }

    /// Sets the temperature modulation gain in Ohms/V.
    #[allow(non_snake_case)]
    pub async fn set_temp_mod_gain_Ohm_V(&mut self, gain_ohm_V: f32) -> Result<()> {
        debug!("set tmodgain: {} Ohms/V", gain_ohm_V);
        self.set("tmodgain", Value::Float(gain_ohm_V)).await
    }

    /// Returns the photodiode current in mA.
    #[allow(non_snake_case)]
    pub async fn pd_current_mA(&mut self) -> Result<f32> {
        let current = self.get::<f32>("iphd").await?;
        debug!("iphd: {} mA", current);
        Ok(current)
    }

    /// Returns the analog input 1 voltage in V.
    #[allow(non_snake_case)]
    pub async fn ain_1_V(&mut self) -> Result<f32> {
        let volts = self.get::<f32>("ain1").await?;
        debug!("ain1: {} V", volts);
        Ok(volts)
    }

    /// Returns the analog input 2 voltage in V.
    #[allow(non_snake_case)]
    pub async fn ain_2_V(&mut self) -> Result<f32> {
        let volts = self.get::<f32>("ain2").await?;
        debug!("ain2: {} V", volts);
        Ok(volts)
    }

    /// Returns the board temperature in C.
    #[allow(non_snake_case)]
    pub async fn board_temp_C(&mut self) -> Result<f32> {
        let temp = self.get::<f32>("tboard").await?;
        debug!("tboard: {} C", temp);
        Ok(temp)
    }

    // TODO(xguo): 128 isn't enough for board status. Let's find other ways to handle this.
    /// Returns a summary of the board status.
    // pub async fn board_status(&mut self) -> Result<BoardStatus> {
    //     let status = self.get::<BoardStatus>("status").await?;
    //     debug!("status: {:?}", status);
    //     Ok(status)
    // }

    /// Saves the current configuration to flash.
    pub async fn save_config(&mut self) -> Result<()> {
        debug!("save");
        // TODO(xguo): I'm keeping in sync with varst here, though "save nothing" feels a bit odd.
        self.set("save", Value::None).await
    }

    /// Returns the serial number of the board.
    pub async fn serial_number(&mut self) -> Result<&'_ [u8]> {
        let serial = self.get::<&'_ [u8]>("serial").await?;
        debug!(
            "serial: {:?}",
            from_utf8(serial).map_err(|_| Error::BytesToUTF8Error)?
        );
        Ok(serial)
    }

    /// Returns the user data string.
    pub async fn userdata<'b>(&mut self) -> Result<&'_ [u8]> {
        let data = self.get::<&'_ [u8]>("userdata").await?;
        debug!(
            "userdata: {:?}",
            from_utf8(data).map_err(|_| Error::BytesToUTF8Error)?
        );
        Ok(data)
    }

    /// Sets the user data string.
    pub async fn set_userdata(&mut self, data: &'_ [u8]) -> Result<()> {
        if !data.is_ascii() || data.iter().any(|&b| b.is_ascii_whitespace()) {
            return Err(Error::DeviceError);
        }
        debug!(
            "userdata write: {:?}",
            from_utf8(data).map_err(|_| Error::BytesToUTF8Error)?
        );
        self.set("userdata write", Value::String(data)).await
    }

    /// Returns the baud rate of the board serial interface.
    #[allow(non_snake_case)]
    pub async fn baud_rate_Hz(&mut self) -> Result<i32> {
        let rate = self.get::<i32>("brate").await?;
        debug!("brate: {} Hz", rate);
        Ok(rate)
    }

    /// Sets the baud rate of the board serial interface.
    #[allow(non_snake_case)]
    pub async fn set_baud_rate_Hz(&mut self, rate_Hz: i32) -> Result<()> {
        debug!("set brate: {} Hz", rate_Hz);
        self.set("brate", Value::Int(rate_Hz)).await
    }

    /// Returns the error state of the board.
    pub async fn err(&mut self) -> Result<i32> {
        let errors = self.get::<i32>("err").await?;
        debug!("err: {}", errors);
        Ok(errors)
    }

    /// Clears the error state of the board.
    pub async fn clear_err(&mut self) -> Result<()> {
        debug!("clear err for CTL200");
        self.set("errclr", Value::None).await
    }

    /// Returns the firmware version.
    pub async fn version(&mut self) -> Result<&[u8]> {
        debug!("Ctl200::version() 0");
        let resp: &[u8] = self.get::<&[u8]>("version").await?;
        let t = core::str::from_utf8(resp).map_err(|_| Error::BytesToUTF8Error)?;
        debug!("version: {:?}", t);
        Ok(resp)
    }

    async fn query(&mut self, request: &str) -> Result<&'_ [u8]> {
        debug!("Sending command: '{}'", request);
        self.uart.write_all(request.as_bytes()).await.map_err(|_| {
            debug!("Failed to write command");
            Error::WriteError
        })?;
        self.uart.write_all(CRLF).await.map_err(|_| {
            debug!("Failed to write CRLF");
            Error::WriteError
        })?;
        self.uart.flush().await.map_err(|_| {
            debug!("Failed to flush UART");
            Error::FlushError
        })?;

        debug!("Waiting for response...");
        let full_response = self.read_until(CRLF_PROMPT).await?;
        debug!("Got response: {:?}", full_response);

        let mut echo_end = None;
        let mut response_start = None;
        let len = full_response.len();

        for i in 0..len - 2 {
            if full_response[i] == b'\r' && full_response[i + 1] == b'\n' {
                if echo_end.is_none() {
                    echo_end = Some(i);
                    response_start = Some(i + 2);
                } else {
                    return Err(Error::InvalidResponse);
                }
            }
        }

        let (echo, response) = match (echo_end, response_start) {
            (Some(e_end), Some(r_start)) => {
                let echo = &full_response[..e_end];
                let response = &full_response[r_start..];
                (echo, response)
            }
            _ => return Err(Error::InvalidResponse),
        };
        debug!("Got echo: {:?}, response: {:?}", echo, response);

        if echo != request.as_bytes() {
            info!(
                "Echo mismatch: expected {}, got {}",
                request,
                core::str::from_utf8(echo).unwrap_or("<invalid>")
            );
            return Err(Error::EchoMismatch);
        }

        Ok(response)
    }

    // TODO(xguo): Implement a more efficient version of this function
    async fn read_until(&mut self, expected_str: &[u8]) -> Result<&'_ [u8]> {
        let mut byte = [0u8; 1];

        while self.buf_pos + expected_str.len() <= MAX_STRING_SIZE {
            // Read one byte
            self.uart
                .read(&mut byte)
                .await
                .map_err(|_| Error::ReadError)?;

            // Add to buffer
            self.buf[self.buf_pos] = byte[0];
            self.buf_pos += 1;

            // Check if we have enough bytes to match expected_str
            if self.buf_pos >= expected_str.len() {
                // Check if buffer ends with expected_str
                let start_idx = self.buf_pos - expected_str.len();
                if self.buf[start_idx..self.buf_pos] == expected_str[..] {
                    // Found match, prepare result
                    let result = &self.buf[..start_idx];

                    // Reset buffer position to allow future reads
                    self.buf_pos = 0;

                    return Ok(result);
                }
            }
        }

        // Buffer overflow
        Err(Error::BufferOverflow)
    }

    async fn get<'b, T>(&'b mut self, param: &str) -> Result<T>
    where
        T: FromBytes<'b>,
    {
        let response = self.query(param).await?;
        T::from_bytes(response)
    }

    #[allow(dead_code)]
    async fn set<'b>(&mut self, param: &str, value: Value<'b>) -> Result<()> {
        use core::fmt::Write;
        let mut s: String<MAX_STRING_SIZE> = String::new();
        write!(&mut s, "{} {}", param, value).unwrap();
        let _ = self.query(&s).await?;
        Ok(())
    }
}

pub enum ParseError {
    ParseError,
}

pub struct NumericBool(pub bool);

impl FromStr for NumericBool {
    type Err = ParseError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match s {
            "0" => Ok(NumericBool(false)),
            "1" => Ok(NumericBool(true)),
            _ => Err(ParseError::ParseError),
        }
    }
}

#[derive(Debug)]
pub enum Value<'a> {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(&'a [u8]),
    None,
}

impl<'a> Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", if *b { "1" } else { "0" }),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", core::str::from_utf8(s).map_err(|_| fmt::Error)?),
            Value::None => write!(f, "None"),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::{collections::HashMap, println, string::String as StdString, sync::Arc, vec::Vec};

    use embedded_io_async::{Read, Write};
    use futures::lock::Mutex;

    use super::*;

    const UNKNOWN_COMMAND: &[u8] = b"Unknown command";

    lazy_static::lazy_static! {
        static ref COMMAND_MAP: Mutex<HashMap<&'static str, StdString>> = {
            let mut m = HashMap::new();
            m.insert("version", StdString::from("V0.17"));
            m.insert("lason", StdString::from("0"));
            // Add more commands as needed
            Mutex::new(m)
        };
    }

    #[derive(Debug)]
    enum MockError {
        WriteError,
    }

    impl embedded_io::Error for MockError {
        fn kind(&self) -> embedded_io::ErrorKind {
            embedded_io::ErrorKind::Other
        }
    }

    #[derive(Clone)]
    struct MockStream {
        read_data: Arc<Mutex<Vec<u8>>>,
        write_data: Arc<Mutex<Vec<u8>>>,
    }

    impl embedded_io::ErrorType for MockStream {
        type Error = MockError;
    }

    impl MockStream {
        fn new() -> Self {
            MockStream {
                read_data: Arc::new(Mutex::new(Vec::new())),
                write_data: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn append_read_data(&self, data: &[u8]) {
            let mut read_data = futures::executor::block_on(self.read_data.lock());
            println!("Appending read data: {:?}", data);
            read_data.extend_from_slice(data);
        }
    }

    impl Read for MockStream {
        async fn read(&mut self, buf: &mut [u8]) -> core::result::Result<usize, Self::Error> {
            let mut data = self.read_data.lock().await;
            let len = data.len().min(buf.len());
            buf[..len].copy_from_slice(&data[..len]);
            data.drain(..len);
            Ok(len)
        }
    }

    impl Write for MockStream {
        async fn write(&mut self, buf: &[u8]) -> core::result::Result<usize, Self::Error> {
            let mut data = self.write_data.lock().await;
            data.extend_from_slice(buf);

            // Check if the data ends with CRLF (Carriage Return and Line Feed)
            if data.ends_with(CRLF) {
                // Extract the command from the data, excluding the CRLF
                let command = StdString::from_utf8(data[..data.len() - 2].to_vec()).unwrap();
                debug!("Received command: {}", command);

                // Append the command and CRLF to the read data
                self.append_read_data(command.as_bytes());
                self.append_read_data(CRLF);
                data.clear();

                let cmds: Vec<StdString> =
                    command.split_whitespace().map(StdString::from).collect();
                match cmds.len() {
                    1 => {
                        // GET command
                        if let Some(response) = COMMAND_MAP.lock().await.get(cmds[0].as_str()) {
                            debug!("Found response for command: {}", response);
                            self.append_read_data(response.as_bytes());
                        } else {
                            debug!("No predefined response for command: {}", cmds[0]);
                            self.append_read_data(UNKNOWN_COMMAND);
                        }
                    }
                    2 => {
                        // SET command
                        if let Some(response) = COMMAND_MAP.lock().await.get_mut(cmds[0].as_str()) {
                            *response = StdString::from(cmds[1].as_str());
                            self.append_read_data(cmds[1].as_bytes());
                        } else {
                            self.append_read_data(UNKNOWN_COMMAND);
                        }
                    }
                    _ => return Err(MockError::WriteError),
                }
                self.append_read_data(CRLF_PROMPT);
            }

            Ok(buf.len())
        }

        async fn flush(&mut self) -> core::result::Result<(), Self::Error> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_ctl200_get() {
        let mock_stream = MockStream::new();
        let mut ctl200 = Ctl200::new(mock_stream);
        let result = ctl200.get::<&[u8]>("version").await.unwrap();
        assert_eq!(result, b"V0.17");
    }

    #[tokio::test]
    async fn test_ctl200_set() {
        let mock_stream: MockStream = MockStream::new();

        let mut ctl200 = Ctl200::new(mock_stream);

        env_logger::builder().is_test(true).try_init().unwrap();
        debug!(">>>Getting lason as false");
        let t = ctl200.get::<bool>("lason").await.unwrap();
        debug!("test_ctl200_set(): lason: {}", t);
        assert_eq!(t, false);
        debug!(">>>Setting lason to true");
        ctl200.set("lason", Value::Bool(true)).await.unwrap();
        debug!(">>>Getting lason as true");
        assert_eq!(ctl200.get::<bool>("lason").await.unwrap(), true);
    }
}
