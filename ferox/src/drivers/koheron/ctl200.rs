use core::{
    fmt::{self, Display},
    str::FromStr,
};

use bitflags::bitflags;
use embedded_io_async::{Read, Write};
use heapless::String;
use serde::{Deserialize, Serialize};

use crate::{debug, info};

// TODO(xguo): Figure out the correct value for MAX_STRING_SIZE.
pub const MAX_STRING_SIZE: usize = 128;

/// The `FixedSizeString` struct is designed to hold a string with a maximum length defined by `MAX_STRING_SIZE`.
/// It is useful in embedded systems where memory constraints are critical and dynamic memory allocation is not desirable.
///
/// # Examples
///
/// ```rust
/// use heapless::String;
/// use core::str::FromStr;
/// use ferox::drivers::koheron::ctl200::FixedSizeString;
///
/// const MAX_STRING_SIZE: usize = 64;
///
/// let fixed_str = FixedSizeString::from_str("Hello, world!").unwrap();
/// assert_eq!(fixed_str.as_str(), "Hello, world!");
/// ```
#[derive(Debug, PartialEq)]
pub struct FixedSizeString(heapless::String<MAX_STRING_SIZE>);

impl Display for FixedSizeString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for FixedSizeString {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{}", self.as_str());
    }
}

impl FromStr for FixedSizeString {
    type Err = Error;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let mut result: String<MAX_STRING_SIZE> = String::new();
        // `push_str` returns Err if the input does not fit into the fixed-size buffer.
        if result.push_str(s).is_err() {
            return Err(Error::StringTooLongError);
        }
        Ok(FixedSizeString(result))
    }
}

impl FixedSizeString {
    /// Returns the inner string slice.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl core::fmt::Write for FixedSizeString {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.push_str(s).map_err(|_| core::fmt::Error)
    }
}

/// Implementing `Deref` allows treating `FixedSizeString` as a `&str`.
impl core::ops::Deref for FixedSizeString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

#[macro_export]
macro_rules! format_fixed {
    ($($arg:tt)*) => {{
        use core::fmt::Write;

        let mut s = FixedSizeString(heapless::String::<MAX_STRING_SIZE>::new());
        core::write!(&mut s, $($arg)*).unwrap();
        s
    }}
}

/// Driver for CTL200 Laser Controller
///
/// See <https://www.koheron.com/support/user-guides/ctl200/>.
pub struct Ctl200<U>
where
    U: Read + Write + 'static,
{
    _uart: U,
}

const CRLF: &[u8] = b"\r\n";
const PROMPT: &[u8] = b">>";
const CRLF_PROMPT: &[u8] = b"\r\n>>";
const UNKNOWN_COMMAND: &[u8] = b"Unknown command";

impl<U> Ctl200<U>
where
    U: Read + Write + 'static,
{
    /// Returns the enabled state of the laser.
    pub async fn laser_en(&mut self) -> Result<bool> {
        let is_on = self.get::<u32>("lason").await? == 1;
        debug!("lason: {}", is_on);
        Ok(is_on)
    }

    /// Sets the enabled state of the laser.
    pub async fn set_laser_en(&mut self, en: bool) -> Result<()> {
        debug!("set lason: {}", en);
        self.set("lason", Value::Bool(Ctl200Bool(en))).await
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
        let is_on = self.get::<u32>("lckon").await? == 1;
        debug!("lckon: {}", is_on);
        Ok(is_on)
    }

    /// Sets the enabled state of the laser interlock.
    pub async fn set_interlock_en(&mut self, en: bool) -> Result<()> {
        debug!("set lckon: {}", en);
        self.set("lckon", Value::Bool(Ctl200Bool(en))).await
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
        let is_on = self.get::<u32>("tecon").await? == 1;
        debug!("tecon: {}", is_on);
        Ok(is_on)
    }

    /// Sets the enabled state of the TEC.
    pub async fn set_tec_en(&mut self, en: bool) -> Result<()> {
        debug!("set tecon: {}", en);
        self.set("tecon", Value::Bool(Ctl200Bool(en))).await
    }

    /// Returns the enabled state of the temperature protection.
    pub async fn temp_prot_en(&mut self) -> Result<bool> {
        let is_on = self.get::<u32>("tprot").await? == 1;
        debug!("tprot: {}", is_on);
        Ok(is_on)
    }

    /// Sets the enabled state of the temperature protection.
    pub async fn set_temp_prot_en(&mut self, en: bool) -> Result<()> {
        debug!("set tprot: {}", en);
        self.set("tprot", Value::Bool(Ctl200Bool(en))).await
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
    pub async fn serial_number(&mut self) -> Result<FixedSizeString> {
        let serial = self.get::<FixedSizeString>("serial").await?;
        debug!("serial: {}", serial.as_str());
        Ok(serial)
    }

    /// Returns the user data string.
    pub async fn userdata(&mut self) -> Result<FixedSizeString> {
        let data = self.get::<FixedSizeString>("userdata").await?;
        debug!("userdata: {}", data.as_str());
        Ok(data)
    }

    /// Sets the user data string.
    pub async fn set_userdata(&mut self, data: FixedSizeString) -> Result<()> {
        if !data.is_ascii() || data.as_bytes().iter().any(|&b| b.is_ascii_whitespace()) {
            return Err(Error::DeviceError(format_fixed!(
                "The input string `{}` must be ASCII-encoded and must not contain any whitespace characters.",
                data.as_str()
            )));
        }
        debug!("userdata write: {}", data.as_str());
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
    pub async fn err(&mut self) -> Result<Ctl200Errs> {
        let errors = Ctl200Errs::from_bits_retain(self.get::<u32>("err").await?);
        debug!("err: {:?}", errors);
        Ok(errors)
    }

    /// Clears the error state of the board.
    pub async fn clear_err(&mut self) -> Result<()> {
        debug!("clear err for CTL200");
        self.set("errclr", Value::None).await
    }

    /// Returns the firmware version.
    pub async fn version(&mut self) -> Result<FixedSizeString> {
        let resp = self.get::<FixedSizeString>("version").await?;
        debug!("version: {:?}", resp.as_str());
        Ok(resp)
    }
}

impl<U> Ctl200<U>
where
    U: Read + Write + 'static,
{
    pub fn new(uart: U) -> Self {
        Ctl200 { _uart: uart }
    }

    async fn query(&mut self, tx: &str) -> Result<FixedSizeString> {
        debug!("Sending command: '{}'", tx);
        self._uart.write_all(tx.as_bytes()).await.map_err(|_| {
            debug!("Failed to write command");
            Error::WriteError
        })?;
        self._uart.write_all(CRLF).await.map_err(|_| {
            debug!("Failed to write CRLF");
            Error::WriteError
        })?;
        self._uart.flush().await.map_err(|_| {
            debug!("Failed to flush UART");
            Error::FlushError
        })?;

        debug!("Waiting for echo...");
        let echo = self.wait_for_expected_str(CRLF).await?;
        debug!("Received echo: '{}', waiting for response...", echo);
        let response = self.wait_for_expected_str(CRLF_PROMPT).await?;
        debug!("Received response: '{}'", response);

        if echo.as_str() != tx {
            info!("Echo mismatch: expected {}, got {}", tx, echo.as_str());
            return Err(Error::EchoMismatch);
        }

        Ok(response)
    }

    async fn wait_for_expected_str(&mut self, expected_str: &[u8]) -> Result<FixedSizeString> {
        let mut buffer = String::new();
        let mut byte = [0u8; 1];

        loop {
            self._uart
                .read(&mut byte)
                .await
                .map_err(|_| Error::ReadError)?;
            buffer
                .push(byte[0] as char)
                .map_err(|_| Error::BufferOverflow)?;

            if buffer.as_bytes().ends_with(expected_str) {
                buffer.truncate(buffer.len() - expected_str.len());
                return Ok(FixedSizeString(buffer));
            }
        }
    }

    async fn get<F>(&mut self, param: &str) -> Result<F>
    where
        F: FromStr,
        F::Err: Display,
    {
        let res: FixedSizeString = self.query(param).await?;
        debug!("Getting value as type: {}", core::any::type_name::<F>());
        let rx = res.parse::<F>().map_err(|e| {
            Error::InvalidResponse(format_fixed!(
                "Could not parse Ctl200 response to `get: {param}` and returns {res}.\nError: {e}"
            ))
        })?;
        Ok(rx)
    }

    async fn set(&mut self, _param: &str, _value: Value) -> Result<()> {
        use core::fmt::Write;
        let mut s: String<MAX_STRING_SIZE> = String::new();
        write!(&mut s, "{} {}", _param, _value).unwrap();
        debug!("Sending command: '{}'", s.as_str());
        let _ = self.query(&s).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ctl200Bool(pub bool);

impl FromStr for Ctl200Bool {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        debug!("Ctl200Bool :: Parsing value: '{}'", s);
        match s {
            "0" => Ok(Ctl200Bool(false)),
            "1" => Ok(Ctl200Bool(true)),
            _ => Err(Error::InvalidResponse(format_fixed!("Expected 0 or 1"))),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Bool(Ctl200Bool),
    Int(i32),
    Float(f32),
    String(FixedSizeString),
    None,
}

// TODO(xguo): Remove FromStr.
impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        debug!("Value :: Parsing value: '{}'", s);
        if let Ok(b) = s.parse::<Ctl200Bool>() {
            Ok(Value::Bool(b))
        } else if let Ok(i) = s.parse::<i32>() {
            Ok(Value::Int(i))
        } else if let Ok(f) = s.parse::<f32>() {
            Ok(Value::Float(f))
        } else {
            Ok(Value::String(FixedSizeString::from_str(s)?))
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", if b.0 { "1" } else { "0" }),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s.as_str()),
            Value::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    BufferOverflow,
    DeviceError(FixedSizeString),
    EchoMismatch,
    FlushError,
    InvalidFirmwareVersion,
    InvalidResponse(FixedSizeString),
    ReadError,
    StringTooLongError,
    WriteError,
}

#[cfg(feature = "defmt")]
impl defmt::Format for Error {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Error::BufferOverflow => defmt::write!(fmt, "Buffer overflow"),
            Error::EchoMismatch => defmt::write!(fmt, "Echo mismatch"),
            Error::FlushError => defmt::write!(fmt, "Flush error"),
            Error::InvalidResponse(details) => {
                defmt::write!(fmt, "Invalid response: {}", details.as_str())
            }
            Error::ReadError => defmt::write!(fmt, "Read error"),
            Error::WriteError => defmt::write!(fmt, "Write error"),
            Error::StringTooLongError => defmt::write!(fmt, "String too long error"),
            Error::InvalidFirmwareVersion => defmt::write!(fmt, "Invalid firmware version"),
            Error::DeviceError(details) => defmt::write!(fmt, "Device error: {}", details.as_str()),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::BufferOverflow => write!(f, "Buffer overflow"),
            Error::EchoMismatch => write!(f, "Echo mismatch"),
            Error::FlushError => write!(f, "Flush error"),
            Error::InvalidResponse(details) => write!(f, "Invalid response: {}", details.as_str()),
            Error::ReadError => write!(f, "Read error"),
            Error::WriteError => write!(f, "Write error"),
            Error::StringTooLongError => write!(f, "String too long error"),
            Error::InvalidFirmwareVersion => write!(f, "Invalid firmware version"),
            Error::DeviceError(details) => write!(f, "Device error: {}", details.as_str()),
        }
    }
}

impl core::error::Error for Error {}

/// Status summary for Ctl200.
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct BoardStatus {
    pub laser_on: bool,
    pub laser_volts: f64,
    pub tec_amps: f64,
    pub tec_volts: f64,
    pub thermistor_ohms: f64,
    pub photodiode_mA: f64,
    pub aux_in_1_volts: f64,
    pub aux_in_2_volts: f64,
}

impl FromStr for BoardStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use itertools::Itertools;

        #[allow(non_snake_case)]
        let (
            laser_on,
            laser_volts,
            tec_amps,
            tec_volts,
            thermistor_ohms,
            photodiode_mA,
            aux_in_1_volts,
            aux_in_2_volts,
        ) = s.split_ascii_whitespace().collect_tuple().ok_or_else(|| {
            Error::InvalidResponse(format_fixed!(
                "Expected Ctl200 `status` command to return an 8-value board status. Got {s}"
            ))
        })?;

        // Helper to translate possible errors. Has to be a fn because I can't get polymorphism out of let-binding a lambda.
        fn parse_failure<E: Display>(label: &str, value: &str, err: E) -> Error {
            Error::InvalidResponse(format_fixed!(
                "Failed to parse {label} status. Got {value}. Error: {err}"
            ))
        }

        Ok(BoardStatus {
            laser_on: laser_on
                .parse::<u64>()
                .map_err(|e| parse_failure("laser_on", laser_on, e))?
                == 1,
            laser_volts: laser_volts
                .parse::<f64>()
                .map_err(|e| parse_failure("laser_volts", laser_volts, e))?,
            tec_amps: tec_amps
                .parse::<f64>()
                .map_err(|e| parse_failure("tec_amps", tec_amps, e))?,
            tec_volts: tec_volts
                .parse::<f64>()
                .map_err(|e| parse_failure("tec_volts", tec_volts, e))?,
            thermistor_ohms: thermistor_ohms
                .parse::<f64>()
                .map_err(|e| parse_failure("thermistor_ohms", thermistor_ohms, e))?,
            photodiode_mA: photodiode_mA
                .parse::<f64>()
                .map_err(|e| parse_failure("photodiode_mA", photodiode_mA, e))?,
            aux_in_1_volts: aux_in_1_volts
                .parse::<f64>()
                .map_err(|e| parse_failure("aux_in_1_volts", aux_in_1_volts, e))?,
            aux_in_2_volts: aux_in_2_volts
                .parse::<f64>()
                .map_err(|e| parse_failure("aux_in_2_volts", aux_in_2_volts, e))?,
        })
    }
}

impl Display for BoardStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            laser_on,
            laser_volts,
            tec_amps,
            tec_volts,
            thermistor_ohms,
            photodiode_mA,
            aux_in_1_volts,
            aux_in_2_volts,
        } = self;

        let laser_state = if *laser_on { "ON" } else { "OFF" };

        writeln!(f, "Ctl200 board status:")?;
        writeln!(f, "  Laser: {laser_state}({laser_volts:.3}V)")?;
        writeln!(f, "  TEC status: {tec_amps:.3}A@{tec_volts:.3}V")?;
        writeln!(f, "  Thermistor: {thermistor_ohms:.3}Ohms")?;
        writeln!(f, "  Photodiode current: {photodiode_mA:.3}mA")?;
        write!(
            f,
            "  Auxiliary inputs: #1({aux_in_1_volts:.3}V) #2({aux_in_2_volts:.3}V)"
        )?;

        Ok(())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for BoardStatus {
    fn format(&self, fmt: defmt::Formatter) {
        let Self {
            laser_on,
            laser_volts,
            tec_amps,
            tec_volts,
            thermistor_ohms,
            photodiode_mA,
            aux_in_1_volts,
            aux_in_2_volts,
        } = self;

        let laser_state = if *laser_on { "ON" } else { "OFF" };

        defmt::write!(fmt, "Ctl200 board status:\n");
        defmt::write!(
            fmt,
            "{}",
            format_fixed!("  Laser: {laser_state}({laser_volts:.3}V)\n")
        );
        defmt::write!(
            fmt,
            "{}",
            format_fixed!("  TEC status: {tec_amps:.3}A@{tec_volts:.3}V")
        );
        defmt::write!(
            fmt,
            "{}",
            format_fixed!("  Thermistor: {thermistor_ohms:.3}Ohms")
        );
        defmt::write!(
            fmt,
            "{}",
            format_fixed!("  Photodiode current: {photodiode_mA:.3}mA")
        );
        defmt::write!(
            fmt,
            "{}",
            format_fixed!("  Auxiliary inputs: #1({aux_in_1_volts:.3}V) #2({aux_in_2_volts:.3}V)")
        );
    }
}

bitflags! {
    /// Errors reported by the Ctl200.
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Ctl200Errs: u32 {
        const UART_BUFFER_OVERFLOW = 0x1;
        const UART_CMD_BEFORE_PROMPT= 0x1 <<1;
        const LASER_UNDERTEMPERATURE= 0x1 <<2;
        const LASER_OVERTEMPERATURE= 0x1 <<3;
        const CMD_UNKNOWN= 0x1 <<4;
        const CMD_INVALID_ARG= 0x1 <<5;
        const LASER_ON_WHILE_INTERLOCK= 0x1 <<6;
        const INTERLOCK_TRIGGERED= 0x1 <<7;
    }
}

const MAX_ERROR_STRING_SIZE: usize = 256;
impl Display for Ctl200Errs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut errors: String<MAX_ERROR_STRING_SIZE> = String::new();

        if self.contains(Ctl200Errs::UART_BUFFER_OVERFLOW) {
            let _ = errors.push_str("UART_BUFFER_OVERFLOW,");
        }
        if self.contains(Ctl200Errs::UART_CMD_BEFORE_PROMPT) {
            let _ = errors.push_str("UART_CMD_BEFORE_PROMPT,");
        }
        if self.contains(Ctl200Errs::LASER_UNDERTEMPERATURE) {
            let _ = errors.push_str("LASER_UNDERTEMPERATURE,");
        }
        if self.contains(Ctl200Errs::LASER_OVERTEMPERATURE) {
            let _ = errors.push_str("LASER_OVERTEMPERATURE,");
        }
        if self.contains(Ctl200Errs::CMD_UNKNOWN) {
            let _ = errors.push_str("CMD_UNKNOWN,");
        }
        if self.contains(Ctl200Errs::CMD_INVALID_ARG) {
            let _ = errors.push_str("CMD_INVALID_ARG,");
        }
        if self.contains(Ctl200Errs::LASER_ON_WHILE_INTERLOCK) {
            let _ = errors.push_str("LASER_ON_WHILE_INTERLOCK,");
        }
        if self.contains(Ctl200Errs::INTERLOCK_TRIGGERED) {
            let _ = errors.push_str("INTERLOCK_TRIGGERED,");
        }
        errors.truncate(errors.len() - 1);
        write!(f, "{}", errors)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Ctl200Errs {
    fn format(&self, fmt: defmt::Formatter) {
        let mut errors: String<MAX_ERROR_STRING_SIZE> = String::new();

        if self.contains(Ctl200Errs::UART_BUFFER_OVERFLOW) {
            let _ = errors.push_str("UART_BUFFER_OVERFLOW,");
        }
        if self.contains(Ctl200Errs::UART_CMD_BEFORE_PROMPT) {
            let _ = errors.push_str("UART_CMD_BEFORE_PROMPT,");
        }
        if self.contains(Ctl200Errs::LASER_UNDERTEMPERATURE) {
            let _ = errors.push_str("LASER_UNDERTEMPERATURE,");
        }
        if self.contains(Ctl200Errs::LASER_OVERTEMPERATURE) {
            let _ = errors.push_str("LASER_OVERTEMPERATURE,");
        }
        if self.contains(Ctl200Errs::CMD_UNKNOWN) {
            let _ = errors.push_str("CMD_UNKNOWN,");
        }
        if self.contains(Ctl200Errs::CMD_INVALID_ARG) {
            let _ = errors.push_str("CMD_INVALID_ARG,");
        }
        if self.contains(Ctl200Errs::LASER_ON_WHILE_INTERLOCK) {
            let _ = errors.push_str("LASER_ON_WHILE_INTERLOCK,");
        }
        if self.contains(Ctl200Errs::INTERLOCK_TRIGGERED) {
            let _ = errors.push_str("INTERLOCK_TRIGGERED,");
        }
        errors.truncate(errors.len() - 1);
        defmt::write!(fmt, "{}", errors.as_str());
    }
}

pub type Result<T> = core::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    extern crate std;
    use core::fmt::Write as fmtWrite;
    use std::{collections::HashMap, println, string::String as StdString, sync::Arc, vec::Vec};

    use embedded_io_async::{Read, Write};
    use futures::lock::Mutex;
    use itertools::Itertools;
    use log::error;

    use super::*;

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
        WriteError
    }

    impl embedded_io::Error for MockError {
        fn kind(&self) -> embedded_io::ErrorKind {
            embedded_io::ErrorKind::Other
        }
    }

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
            println!(
                "Reading {} bytes from mock stream: '{:?}'",
                len,
                &data[..len]
            );
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

                let cmds: Vec<StdString> = command.split_whitespace().map(StdString::from).collect();
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
        let result: FixedSizeString = ctl200.get("version").await.unwrap();
        assert_eq!(result.as_str(), "V0.17");
    }

    #[tokio::test]
    async fn test_ctl200_set() {
        let read_data = b"OK\r\n".to_vec();
        let mock_stream: MockStream = MockStream::new();

        let mut ctl200 = Ctl200::new(mock_stream);

        env_logger::builder().is_test(true).try_init().unwrap();
        log::info!("xfguo: Setting lason to true");
        println!(">>>Getting lason as false");
        let t = ctl200.get::<Ctl200Bool>("lason").await.unwrap().0;
        debug!("test_ctl200_set(): lason: {}", t);
        assert_eq!(t, false);
        println!(">>>Setting lason to true");
        ctl200.set("lason", Value::Bool(Ctl200Bool(true))).await.unwrap();
        println!(">>>Getting lason as true");
        assert_eq!(ctl200.get::<Ctl200Bool>("lason").await.unwrap().0, true);
    }

    #[test]
    fn test_fixed_size_string_from_str() {
        let fixed_str = FixedSizeString::from_str("Hello, world!").unwrap();
        assert_eq!(fixed_str.as_str(), "Hello, world!");
    }

    #[test]
    fn test_fixed_size_string_from_str_too_long() {
        let long_str = "a".repeat(MAX_STRING_SIZE + 1);
        let result = FixedSizeString::from_str(&long_str);
        assert!(matches!(result, Err(Error::StringTooLongError)));
    }

    #[test]
    fn test_fixed_size_string_write_str() {
        let mut fixed_str = FixedSizeString::from_str("Hello").unwrap();
        write!(fixed_str, ", world!").unwrap();
        assert_eq!(fixed_str.as_str(), "Hello, world!");
    }

    #[test]
    fn test_fixed_size_string_write_str_too_long() {
        let mut fixed_str = FixedSizeString::from_str("Hello").unwrap();
        let long_str = "a".repeat(MAX_STRING_SIZE);
        let result = write!(fixed_str, "{}", long_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_ctl200_errs_display() {
        let mut errors = Ctl200Errs::empty();
        errors.insert(Ctl200Errs::UART_BUFFER_OVERFLOW);
        errors.insert(Ctl200Errs::LASER_OVERTEMPERATURE);
        let error_str = format_fixed!("{}", errors);
        assert_eq!(
            error_str.as_str(),
            "UART_BUFFER_OVERFLOW,LASER_OVERTEMPERATURE"
        );
    }

    #[test]
    fn test_ctl200_errs_from_bits() {
        let errors = Ctl200Errs::from_bits(0x1 | 0x1 << 3).unwrap();
        assert!(errors.contains(Ctl200Errs::UART_BUFFER_OVERFLOW));
        assert!(errors.contains(Ctl200Errs::LASER_OVERTEMPERATURE));
        assert!(!errors.contains(Ctl200Errs::CMD_UNKNOWN));
    }

    #[test]
    fn test_ctl200_errs_all() {
        let all_errors = Ctl200Errs::all();
        assert!(all_errors.contains(Ctl200Errs::UART_BUFFER_OVERFLOW));
        assert!(all_errors.contains(Ctl200Errs::UART_CMD_BEFORE_PROMPT));
        assert!(all_errors.contains(Ctl200Errs::LASER_UNDERTEMPERATURE));
        assert!(all_errors.contains(Ctl200Errs::LASER_OVERTEMPERATURE));
        assert!(all_errors.contains(Ctl200Errs::CMD_UNKNOWN));
        assert!(all_errors.contains(Ctl200Errs::CMD_INVALID_ARG));
        assert!(all_errors.contains(Ctl200Errs::LASER_ON_WHILE_INTERLOCK));
        assert!(all_errors.contains(Ctl200Errs::INTERLOCK_TRIGGERED));
    }

    #[test]
    fn test_fixed_size_string_as_str() {
        let fixed_str = FixedSizeString::from_str("Test string").unwrap();
        assert_eq!(fixed_str.as_str(), "Test string");
    }

    #[test]
    fn test_fixed_size_string_deref() {
        let fixed_str = FixedSizeString::from_str("Deref test").unwrap();
        assert_eq!(&*fixed_str, "Deref test");
    }

    #[test]
    fn test_fixed_size_string_empty() {
        let fixed_str = FixedSizeString::from_str("").unwrap();
        assert_eq!(fixed_str.as_str(), "");
    }

    #[test]
    fn test_fixed_size_string_partial_eq() {
        let str1 = FixedSizeString::from_str("Test").unwrap();
        let str2 = FixedSizeString::from_str("Test").unwrap();
        let str3 = FixedSizeString::from_str("Different").unwrap();
        assert_eq!(str1, str2);
        assert_ne!(str1, str3);
    }

    #[test]
    fn test_value_from_str_bool() {
        let value = Value::from_str("1").unwrap();
        assert_eq!(value, Value::Bool(Ctl200Bool(true)));

        let value = Value::from_str("0").unwrap();
        assert_eq!(value, Value::Bool(Ctl200Bool(false)));
    }

    #[test]
    fn test_value_from_str_int() {
        let value = Value::from_str("42").unwrap();
        assert_eq!(value, Value::Int(42));
    }

    #[test]
    fn test_value_from_str_float() {
        let value = Value::from_str("3.14").unwrap();
        assert_eq!(value, Value::Float(3.14));
    }

    #[test]
    fn test_value_from_str_string() {
        let value = Value::from_str("Hello, world!").unwrap();
        assert_eq!(
            value,
            Value::String(FixedSizeString::from_str("Hello, world!").unwrap())
        );
    }

    #[test]
    fn test_value_from_str_invalid() {
        let result = Value::from_str("invalid");
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_value_display() {
        let value = Value::Bool(Ctl200Bool(true));
        assert_eq!(format_fixed!("{}", value).as_str(), "1");

        let value = Value::Int(42);
        assert_eq!(format_fixed!("{}", value).as_str(), "42");

        let value = Value::Float(3.14);
        assert_eq!(format_fixed!("{}", value).as_str(), "3.14");

        let value = Value::String(FixedSizeString::from_str("Hello, world!").unwrap());
        assert_eq!(format_fixed!("{}", value).as_str(), "Hello, world!");

        let value = Value::None;
        assert_eq!(format_fixed!("{}", value).as_str(), "None");
    }

    #[test]
    fn test_error_display() {
        let error = Error::BufferOverflow;
        assert_eq!(format_fixed!("{}", error).as_str(), "Buffer overflow");

        let error = Error::EchoMismatch;
        assert_eq!(format_fixed!("{}", error).as_str(), "Echo mismatch");

        let error = Error::FlushError;
        assert_eq!(format_fixed!("{}", error).as_str(), "Flush error");

        let error = Error::InvalidFirmwareVersion;
        assert_eq!(
            format_fixed!("{}", error).as_str(),
            "Invalid firmware version"
        );

        let error = Error::InvalidResponse(FixedSizeString::from_str("Invalid response").unwrap());
        assert_eq!(
            format_fixed!("{}", error).as_str(),
            "Invalid response: Invalid response"
        );

        let error = Error::ReadError;
        assert_eq!(format_fixed!("{}", error).as_str(), "Read error");

        let error = Error::StringTooLongError;
        assert_eq!(format_fixed!("{}", error).as_str(), "String too long error");

        let error = Error::WriteError;
        assert_eq!(format_fixed!("{}", error).as_str(), "Write error");

        let error = Error::DeviceError(FixedSizeString::from_str("Device error").unwrap());
        assert_eq!(
            format_fixed!("{}", error).as_str(),
            "Device error: Device error"
        );
    }

    #[test]
    fn test_error_from_str() {
        let error = Error::InvalidResponse(FixedSizeString::from_str("Invalid response").unwrap());
        assert_eq!(
            format_fixed!("{}", error).as_str(),
            "Invalid response: Invalid response"
        );

        let error = Error::DeviceError(FixedSizeString::from_str("Device error").unwrap());
        assert_eq!(
            format_fixed!("{}", error).as_str(),
            "Device error: Device error"
        );
    }
    #[test]
    fn test_error_partial_eq() {
        let error1 = Error::BufferOverflow;
        let error2 = Error::BufferOverflow;
        let error3 = Error::ReadError;
        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }

    #[test]
    fn test_board_status_display() {
        let status = BoardStatus {
            laser_on: true,
            laser_volts: 3.14,
            tec_amps: 1.23,
            tec_volts: 4.56,
            thermistor_ohms: 10000.0,
            photodiode_mA: 0.789,
            aux_in_1_volts: 2.345,
            aux_in_2_volts: 3.456,
        };

        let expected: &str = "\
Ctl200 board status:
  Laser: ON(3.140V)
  TEC status: 1.230A@4.560V
  Thermistor: 10000.000Ohms
  Photodiode current: 0.789mA
  Auxiliary inputs: #1(2.345V) #2(3.456V)";

        assert_eq!(format_fixed!("{}", status).as_str(), expected);
    }
}

pub use format_fixed;