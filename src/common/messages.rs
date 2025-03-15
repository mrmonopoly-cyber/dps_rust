// Generated code!
#![allow(unused_comparisons, unreachable_patterns)]
#![allow(clippy::let_and_return, clippy::eq_op)]
#![allow(clippy::excessive_precision, clippy::manual_range_contains, clippy::absurd_extreme_comparisons)]
#![deny(clippy::arithmetic_side_effects)]

//! Message definitions from file `"dps_mesages.dbc"`
//!
//! - Version: `Version("1.0")`

use core::ops::BitOr;
use bitvec::prelude::*;
#[cfg(feature = "arb")]
use arbitrary::{Arbitrary, Unstructured};

/// All messages
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum Messages {
    /// DpsSlaveMex
    DpsSlaveMex(DpsSlaveMex),
    /// DpsMasterMex
    DpsMasterMex(DpsMasterMex),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: u32, payload: &[u8]) -> Result<Self, CanError> {
        
        let res = match id {
            650 => Messages::DpsSlaveMex(DpsSlaveMex::try_from(payload)?),
            651 => Messages::DpsMasterMex(DpsMasterMex::try_from(payload)?),
            n => return Err(CanError::UnknownMessageId(n)),
        };
        Ok(res)
    }
}

/// DpsSlaveMex
///
/// - ID: 650 (0x28a)
/// - Size: 8 bytes
/// - Transmitter: SLAVE
#[derive(Clone, Copy)]
pub struct DpsSlaveMex {
    raw: [u8; 8],
}

impl DpsSlaveMex {
    pub const MESSAGE_ID: u32 = 650;
    
    pub const BOARD_ID_MIN: u8 = 0_u8;
    pub const BOARD_ID_MAX: u8 = 15_u8;
    pub const MODE_MIN: u8 = 0_u8;
    pub const MODE_MAX: u8 = 15_u8;
    pub const ID_MIN: u8 = 0_u8;
    pub const ID_MAX: u8 = 15_u8;
    pub const BOARD_NAME_MIN: u64 = 0_u64;
    pub const BOARD_NAME_MAX: u64 = 0_u64;
    pub const INFO_VAR_ID_MIN: u8 = 0_u8;
    pub const INFO_VAR_ID_MAX: u8 = 15_u8;
    pub const VAR_NAME_MIN: u64 = 0_u64;
    pub const VAR_NAME_MAX: u64 = 0_u64;
    pub const VALUE_VAR_ID_MIN: u8 = 0_u8;
    pub const VALUE_VAR_ID_MAX: u8 = 15_u8;
    pub const XTYPE_MIN: u8 = 0_u8;
    pub const XTYPE_MAX: u8 = 2_u8;
    pub const SIZE_MIN: u8 = 0_u8;
    pub const SIZE_MAX: u8 = 2_u8;
    pub const VAR_ID_MIN: u8 = 0_u8;
    pub const VAR_ID_MAX: u8 = 15_u8;
    pub const VALUE_MIN: u32 = 0_u32;
    pub const VALUE_MAX: u32 = 2_u32;
    
    /// Construct new DpsSlaveMex from values
    pub fn new(board_id: u8, mode: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_board_id(board_id)?;
        res.set_mode(mode)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// board_id
    ///
    /// - Min: 0
    /// - Max: 15
    /// - Unit: "slave board id"
    /// - Receivers: MASTER
    #[inline(always)]
    pub fn board_id(&self) -> u8 {
        self.board_id_raw()
    }
    
    /// Get raw value of board_id
    ///
    /// - Start bit: 0
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn board_id_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<u8>();
        
        signal
    }
    
    /// Set value of board_id
    #[inline(always)]
    pub fn set_board_id(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 650 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    
    /// Get raw value of Mode
    ///
    /// - Start bit: 4
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mode_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[4..8].load_le::<u8>();
        
        signal
    }
    
    pub fn mode(&mut self) -> Result<DpsSlaveMexMode, CanError> {
        match self.mode_raw() {
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: 650, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of Mode
    #[inline(always)]
    fn set_mode(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 650 });
        }
        self.raw.view_bits_mut::<Lsb0>()[4..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for DpsSlaveMex {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for DpsSlaveMex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("DpsSlaveMex")
                .field("board_id", &self.board_id())
            .finish()
        } else {
            f.debug_tuple("DpsSlaveMex").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for DpsSlaveMex {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let board_id = u.int_in_range(0..=15)?;
        let mode = u.int_in_range(0..=15)?;
        DpsSlaveMex::new(board_id,mode).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for type
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum DpsSlaveMexType {
    Integer,
    Float,
    String,
    _Other(u8),
}

impl From<DpsSlaveMexType> for u8 {
    fn from(val: DpsSlaveMexType) -> u8 {
        match val {
            DpsSlaveMexType::Integer => 0,
            DpsSlaveMexType::Float => 1,
            DpsSlaveMexType::String => 2,
            DpsSlaveMexType::_Other(x) => x,
        }
    }
}

/// Defined values for size
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum DpsSlaveMexSize {
    X8bit,
    X16bit,
    X32bit,
    _Other(u8),
}

impl From<DpsSlaveMexSize> for u8 {
    fn from(val: DpsSlaveMexSize) -> u8 {
        match val {
            DpsSlaveMexSize::X8bit => 0,
            DpsSlaveMexSize::X16bit => 1,
            DpsSlaveMexSize::X32bit => 2,
            DpsSlaveMexSize::_Other(x) => x,
        }
    }
}

/// Defined values for multiplexed signal DpsSlaveMex
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum DpsSlaveMexMode {
}


/// DpsMasterMex
///
/// - ID: 651 (0x28b)
/// - Size: 8 bytes
/// - Transmitter: MASTER
#[derive(Clone, Copy)]
pub struct DpsMasterMex {
    raw: [u8; 8],
}

impl DpsMasterMex {
    pub const MESSAGE_ID: u32 = 651;
    
    pub const MODE_MIN: u8 = 0_u8;
    pub const MODE_MAX: u8 = 15_u8;
    pub const VAR_NAME_BOARD_ID_MIN: u8 = 0_u8;
    pub const VAR_NAME_BOARD_ID_MAX: u8 = 15_u8;
    pub const VAR_METADATA_BOARD_ID_MIN: u8 = 0_u8;
    pub const VAR_METADATA_BOARD_ID_MAX: u8 = 15_u8;
    pub const VAR_METADATA_VAR_ID_MIN: u8 = 0_u8;
    pub const VAR_METADATA_VAR_ID_MAX: u8 = 15_u8;
    pub const VAR_VALUE_BOARD_ID_MIN: u8 = 0_u8;
    pub const VAR_VALUE_BOARD_ID_MAX: u8 = 15_u8;
    pub const VAR_VALUE_VAR_ID_MIN: u8 = 0_u8;
    pub const VAR_VALUE_VAR_ID_MAX: u8 = 15_u8;
    
    /// Construct new DpsMasterMex from values
    pub fn new(mode: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_mode(mode)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Get raw value of Mode
    ///
    /// - Start bit: 0
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mode_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<u8>();
        
        signal
    }
    
    pub fn mode(&mut self) -> Result<DpsMasterMexMode, CanError> {
        match self.mode_raw() {
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: 651, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of Mode
    #[inline(always)]
    fn set_mode(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 651 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for DpsMasterMex {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for DpsMasterMex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("DpsMasterMex")
            .finish()
        } else {
            f.debug_tuple("DpsMasterMex").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for DpsMasterMex {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let mode = u.int_in_range(0..=15)?;
        DpsMasterMex::new(mode).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for multiplexed signal DpsMasterMex
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum DpsMasterMexMode {
}



/// This is just to make testing easier
#[allow(dead_code)]
fn main() {}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(any(feature = "debug", feature = "std"), derive(Debug))]
pub enum CanError {
    UnknownMessageId(u32),
    /// Signal parameter is not within the range
    /// defined in the dbc
    ParameterOutOfRange {
        /// dbc message id
        message_id: u32,
    },
    InvalidPayloadSize,
    /// Multiplexor value not defined in the dbc
    InvalidMultiplexor {
        /// dbc message id
        message_id: u32,
        /// Multiplexor value not defined in the dbc
        multiplexor: u16,
    },
}

#[cfg(feature = "std")]
use std::error::Error;
#[cfg(feature = "std")]
use std::fmt;

#[cfg(feature = "std")]
impl fmt::Display for CanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl Error for CanError {}
#[cfg(feature = "arb")]
trait UnstructuredFloatExt {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32>;
}

#[cfg(feature = "arb")]
impl UnstructuredFloatExt for arbitrary::Unstructured<'_> {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32> {
        let min = range.start();
        let max = range.end();
        let steps = u32::MAX;
        let factor = (max - min) / (steps as f32);
        let random_int: u32 = self.int_in_range(0..=steps)?;
        let random = min + factor * (random_int as f32);
        Ok(random)
    }
}

