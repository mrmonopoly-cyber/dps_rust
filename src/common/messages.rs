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
    pub const VALUE_VAR_TYPE_MIN: u8 = 0_u8;
    pub const VALUE_VAR_TYPE_MAX: u8 = 2_u8;
    pub const VALUE_VAR_SIZE_MIN: u8 = 0_u8;
    pub const VALUE_VAR_SIZE_MAX: u8 = 2_u8;
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
    /// - Receivers: VECTOR_XXX
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
            0 => Ok(DpsSlaveMexMode::M0(DpsSlaveMexModeM0{ raw: self.raw })),
            1 => Ok(DpsSlaveMexMode::M1(DpsSlaveMexModeM1{ raw: self.raw })),
            2 => Ok(DpsSlaveMexMode::M2(DpsSlaveMexModeM2{ raw: self.raw })),
            3 => Ok(DpsSlaveMexMode::M3(DpsSlaveMexModeM3{ raw: self.raw })),
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
    
    /// Set value of Mode
    #[inline(always)]
    pub fn set_m0(&mut self, value: DpsSlaveMexModeM0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mode(0)?;
        Ok(())
    }
    
    /// Set value of Mode
    #[inline(always)]
    pub fn set_m1(&mut self, value: DpsSlaveMexModeM1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mode(1)?;
        Ok(())
    }
    
    /// Set value of Mode
    #[inline(always)]
    pub fn set_m2(&mut self, value: DpsSlaveMexModeM2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mode(2)?;
        Ok(())
    }
    
    /// Set value of Mode
    #[inline(always)]
    pub fn set_m3(&mut self, value: DpsSlaveMexModeM3) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mode(3)?;
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
/// Defined values for value_var_type
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum DpsSlaveMexValueVarType {
    Unsigned,
    Signed,
    Floated,
    _Other(u8),
}

impl From<DpsSlaveMexValueVarType> for u8 {
    fn from(val: DpsSlaveMexValueVarType) -> u8 {
        match val {
            DpsSlaveMexValueVarType::Unsigned => 0,
            DpsSlaveMexValueVarType::Signed => 1,
            DpsSlaveMexValueVarType::Floated => 2,
            DpsSlaveMexValueVarType::_Other(x) => x,
        }
    }
}

/// Defined values for value_var_size
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum DpsSlaveMexValueVarSize {
    X8Bit,
    X16Bit,
    X32Bit,
    _Other(u8),
}

impl From<DpsSlaveMexValueVarSize> for u8 {
    fn from(val: DpsSlaveMexValueVarSize) -> u8 {
        match val {
            DpsSlaveMexValueVarSize::X8Bit => 0,
            DpsSlaveMexValueVarSize::X16Bit => 1,
            DpsSlaveMexValueVarSize::X32Bit => 2,
            DpsSlaveMexValueVarSize::_Other(x) => x,
        }
    }
}

/// Defined values for multiplexed signal DpsSlaveMex
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum DpsSlaveMexMode {
    M0(DpsSlaveMexModeM0),
    M1(DpsSlaveMexModeM1),
    M2(DpsSlaveMexModeM2),
    M3(DpsSlaveMexModeM3),
}

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct DpsSlaveMexModeM0 { raw: [u8; 8] }

impl DpsSlaveMexModeM0 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// id
///
/// - Min: 0
/// - Max: 15
/// - Unit: "slave board id"
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn id(&self) -> u8 {
    self.id_raw()
}

/// Get raw value of id
///
/// - Start bit: 8
/// - Signal size: 4 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn id_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..12].load_le::<u8>();
    
    signal
}

/// Set value of id
#[inline(always)]
pub fn set_id(&mut self, value: u8) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_u8 || 15_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 650 });
    }
    self.raw.view_bits_mut::<Lsb0>()[8..12].store_le(value);
    Ok(())
}

/// board_name
///
/// - Min: 0
/// - Max: 0
/// - Unit: "slave board name"
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn board_name(&self) -> u64 {
    self.board_name_raw()
}

/// Get raw value of board_name
///
/// - Start bit: 16
/// - Signal size: 48 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn board_name_raw(&self) -> u64 {
    let signal = self.raw.view_bits::<Lsb0>()[16..64].load_le::<u64>();
    
    signal
}

/// Set value of board_name
#[inline(always)]
pub fn set_board_name(&mut self, value: u64) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_u64 || 0_u64 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 650 });
    }
    self.raw.view_bits_mut::<Lsb0>()[16..64].store_le(value);
    Ok(())
}

}

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct DpsSlaveMexModeM1 { raw: [u8; 8] }

impl DpsSlaveMexModeM1 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// info_var_id
///
/// - Min: 0
/// - Max: 15
/// - Unit: "slave var id"
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn info_var_id(&self) -> u8 {
    self.info_var_id_raw()
}

/// Get raw value of info_var_id
///
/// - Start bit: 8
/// - Signal size: 4 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn info_var_id_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..12].load_le::<u8>();
    
    signal
}

/// Set value of info_var_id
#[inline(always)]
pub fn set_info_var_id(&mut self, value: u8) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_u8 || 15_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 650 });
    }
    self.raw.view_bits_mut::<Lsb0>()[8..12].store_le(value);
    Ok(())
}

/// var_name
///
/// - Min: 0
/// - Max: 0
/// - Unit: "slave var name"
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn var_name(&self) -> u64 {
    self.var_name_raw()
}

/// Get raw value of var_name
///
/// - Start bit: 16
/// - Signal size: 48 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn var_name_raw(&self) -> u64 {
    let signal = self.raw.view_bits::<Lsb0>()[16..64].load_le::<u64>();
    
    signal
}

/// Set value of var_name
#[inline(always)]
pub fn set_var_name(&mut self, value: u64) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_u64 || 0_u64 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 650 });
    }
    self.raw.view_bits_mut::<Lsb0>()[16..64].store_le(value);
    Ok(())
}

}

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct DpsSlaveMexModeM2 { raw: [u8; 8] }

impl DpsSlaveMexModeM2 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// value_var_id
///
/// - Min: 0
/// - Max: 15
/// - Unit: "slave var id"
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn value_var_id(&self) -> u8 {
    self.value_var_id_raw()
}

/// Get raw value of value_var_id
///
/// - Start bit: 8
/// - Signal size: 4 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn value_var_id_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..12].load_le::<u8>();
    
    signal
}

/// Set value of value_var_id
#[inline(always)]
pub fn set_value_var_id(&mut self, value: u8) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_u8 || 15_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 650 });
    }
    self.raw.view_bits_mut::<Lsb0>()[8..12].store_le(value);
    Ok(())
}

/// value_var_type
///
/// - Min: 0
/// - Max: 2
/// - Unit: "slave var type"
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn value_var_type(&self) -> DpsSlaveMexValueVarType {
    let signal = self.raw.view_bits::<Lsb0>()[12..14].load_le::<u8>();
    
    match signal {
        0 => DpsSlaveMexValueVarType::Unsigned,
        1 => DpsSlaveMexValueVarType::Signed,
        2 => DpsSlaveMexValueVarType::Floated,
        _ => DpsSlaveMexValueVarType::_Other(self.value_var_type_raw()),
    }
}

/// Get raw value of value_var_type
///
/// - Start bit: 12
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn value_var_type_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[12..14].load_le::<u8>();
    
    signal
}

/// Set value of value_var_type
#[inline(always)]
pub fn set_value_var_type(&mut self, value: u8) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_u8 || 2_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 650 });
    }
    self.raw.view_bits_mut::<Lsb0>()[12..14].store_le(value);
    Ok(())
}

/// value_var_size
///
/// - Min: 0
/// - Max: 2
/// - Unit: "slave var size"
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn value_var_size(&self) -> DpsSlaveMexValueVarSize {
    let signal = self.raw.view_bits::<Lsb0>()[14..16].load_le::<u8>();
    
    match signal {
        0 => DpsSlaveMexValueVarSize::X8Bit,
        1 => DpsSlaveMexValueVarSize::X16Bit,
        2 => DpsSlaveMexValueVarSize::X32Bit,
        _ => DpsSlaveMexValueVarSize::_Other(self.value_var_size_raw()),
    }
}

/// Get raw value of value_var_size
///
/// - Start bit: 14
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn value_var_size_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[14..16].load_le::<u8>();
    
    signal
}

/// Set value of value_var_size
#[inline(always)]
pub fn set_value_var_size(&mut self, value: u8) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_u8 || 2_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 650 });
    }
    self.raw.view_bits_mut::<Lsb0>()[14..16].store_le(value);
    Ok(())
}

}

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct DpsSlaveMexModeM3 { raw: [u8; 8] }

impl DpsSlaveMexModeM3 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// var_id
///
/// - Min: 0
/// - Max: 15
/// - Unit: "slave var id"
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn var_id(&self) -> u8 {
    self.var_id_raw()
}

/// Get raw value of var_id
///
/// - Start bit: 8
/// - Signal size: 4 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn var_id_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..12].load_le::<u8>();
    
    signal
}

/// Set value of var_id
#[inline(always)]
pub fn set_var_id(&mut self, value: u8) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_u8 || 15_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 650 });
    }
    self.raw.view_bits_mut::<Lsb0>()[8..12].store_le(value);
    Ok(())
}

/// value
///
/// - Min: 0
/// - Max: 2
/// - Unit: "slave var value"
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn value(&self) -> u32 {
    self.value_raw()
}

/// Get raw value of value
///
/// - Start bit: 16
/// - Signal size: 32 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn value_raw(&self) -> u32 {
    let signal = self.raw.view_bits::<Lsb0>()[16..48].load_le::<u32>();
    
    signal
}

/// Set value of value
#[inline(always)]
pub fn set_value(&mut self, value: u32) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_u32 || 2_u32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 650 });
    }
    self.raw.view_bits_mut::<Lsb0>()[16..48].store_le(value);
    Ok(())
}

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
    pub const VAR_REFRESH_BOARD_ID_MIN: u8 = 0_u8;
    pub const VAR_REFRESH_BOARD_ID_MAX: u8 = 15_u8;
    pub const VAR_REFRESH_VAR_ID_MIN: u8 = 0_u8;
    pub const VAR_REFRESH_VAR_ID_MAX: u8 = 15_u8;
    pub const VAR_VALUE_BOARD_ID_MIN: u8 = 0_u8;
    pub const VAR_VALUE_BOARD_ID_MAX: u8 = 15_u8;
    pub const VAR_VALUE_VAR_ID_MIN: u8 = 0_u8;
    pub const VAR_VALUE_VAR_ID_MAX: u8 = 15_u8;
    pub const VALUE_MIN: u32 = 0_u32;
    pub const VALUE_MAX: u32 = 0_u32;
    
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
            0 => Ok(DpsMasterMexMode::M0(DpsMasterMexModeM0{ raw: self.raw })),
            1 => Ok(DpsMasterMexMode::M1(DpsMasterMexModeM1{ raw: self.raw })),
            2 => Ok(DpsMasterMexMode::M2(DpsMasterMexModeM2{ raw: self.raw })),
            3 => Ok(DpsMasterMexMode::M3(DpsMasterMexModeM3{ raw: self.raw })),
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
    
    /// Set value of Mode
    #[inline(always)]
    pub fn set_m0(&mut self, value: DpsMasterMexModeM0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mode(0)?;
        Ok(())
    }
    
    /// Set value of Mode
    #[inline(always)]
    pub fn set_m1(&mut self, value: DpsMasterMexModeM1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mode(1)?;
        Ok(())
    }
    
    /// Set value of Mode
    #[inline(always)]
    pub fn set_m2(&mut self, value: DpsMasterMexModeM2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mode(2)?;
        Ok(())
    }
    
    /// Set value of Mode
    #[inline(always)]
    pub fn set_m3(&mut self, value: DpsMasterMexModeM3) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mode(3)?;
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
    M0(DpsMasterMexModeM0),
    M1(DpsMasterMexModeM1),
    M2(DpsMasterMexModeM2),
    M3(DpsMasterMexModeM3),
}

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct DpsMasterMexModeM0 { raw: [u8; 8] }

impl DpsMasterMexModeM0 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// reserved
///
/// - Min: 0
/// - Max: 0
/// - Unit: ""
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn reserved(&self) -> bool {
    self.reserved_raw()
}

/// Get raw value of reserved
///
/// - Start bit: 4
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn reserved_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[4..5].load_le::<u8>();
    
    signal == 1
}

/// Set value of reserved
#[inline(always)]
pub fn set_reserved(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[4..5].store_le(value);
    Ok(())
}

}

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct DpsMasterMexModeM1 { raw: [u8; 8] }

impl DpsMasterMexModeM1 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// var_name_board_id
///
/// - Min: 0
/// - Max: 15
/// - Unit: "slave board id"
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn var_name_board_id(&self) -> u8 {
    self.var_name_board_id_raw()
}

/// Get raw value of var_name_board_id
///
/// - Start bit: 4
/// - Signal size: 4 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn var_name_board_id_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[4..8].load_le::<u8>();
    
    signal
}

/// Set value of var_name_board_id
#[inline(always)]
pub fn set_var_name_board_id(&mut self, value: u8) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_u8 || 15_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 651 });
    }
    self.raw.view_bits_mut::<Lsb0>()[4..8].store_le(value);
    Ok(())
}

}

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct DpsMasterMexModeM2 { raw: [u8; 8] }

impl DpsMasterMexModeM2 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// var_refresh_board_id
///
/// - Min: 0
/// - Max: 15
/// - Unit: "slave board id"
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn var_refresh_board_id(&self) -> u8 {
    self.var_refresh_board_id_raw()
}

/// Get raw value of var_refresh_board_id
///
/// - Start bit: 4
/// - Signal size: 4 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn var_refresh_board_id_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[4..8].load_le::<u8>();
    
    signal
}

/// Set value of var_refresh_board_id
#[inline(always)]
pub fn set_var_refresh_board_id(&mut self, value: u8) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_u8 || 15_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 651 });
    }
    self.raw.view_bits_mut::<Lsb0>()[4..8].store_le(value);
    Ok(())
}

/// var_refresh_var_id
///
/// - Min: 0
/// - Max: 15
/// - Unit: "slave var id"
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn var_refresh_var_id(&self) -> u8 {
    self.var_refresh_var_id_raw()
}

/// Get raw value of var_refresh_var_id
///
/// - Start bit: 8
/// - Signal size: 4 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn var_refresh_var_id_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..12].load_le::<u8>();
    
    signal
}

/// Set value of var_refresh_var_id
#[inline(always)]
pub fn set_var_refresh_var_id(&mut self, value: u8) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_u8 || 15_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 651 });
    }
    self.raw.view_bits_mut::<Lsb0>()[8..12].store_le(value);
    Ok(())
}

}

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct DpsMasterMexModeM3 { raw: [u8; 8] }

impl DpsMasterMexModeM3 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// var_value_board_id
///
/// - Min: 0
/// - Max: 15
/// - Unit: "slave board id"
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn var_value_board_id(&self) -> u8 {
    self.var_value_board_id_raw()
}

/// Get raw value of var_value_board_id
///
/// - Start bit: 4
/// - Signal size: 4 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn var_value_board_id_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[4..8].load_le::<u8>();
    
    signal
}

/// Set value of var_value_board_id
#[inline(always)]
pub fn set_var_value_board_id(&mut self, value: u8) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_u8 || 15_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 651 });
    }
    self.raw.view_bits_mut::<Lsb0>()[4..8].store_le(value);
    Ok(())
}

/// var_value_var_id
///
/// - Min: 0
/// - Max: 15
/// - Unit: "slave var id"
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn var_value_var_id(&self) -> u8 {
    self.var_value_var_id_raw()
}

/// Get raw value of var_value_var_id
///
/// - Start bit: 8
/// - Signal size: 4 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn var_value_var_id_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..12].load_le::<u8>();
    
    signal
}

/// Set value of var_value_var_id
#[inline(always)]
pub fn set_var_value_var_id(&mut self, value: u8) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_u8 || 15_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 651 });
    }
    self.raw.view_bits_mut::<Lsb0>()[8..12].store_le(value);
    Ok(())
}

/// value
///
/// - Min: 0
/// - Max: 0
/// - Unit: "slave var value"
/// - Receivers: VECTOR_XXX
#[inline(always)]
pub fn value(&self) -> u32 {
    self.value_raw()
}

/// Get raw value of value
///
/// - Start bit: 16
/// - Signal size: 32 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn value_raw(&self) -> u32 {
    let signal = self.raw.view_bits::<Lsb0>()[16..48].load_le::<u32>();
    
    signal
}

/// Set value of value
#[inline(always)]
pub fn set_value(&mut self, value: u32) -> Result<(), CanError> {
    #[cfg(feature = "range_checked")]
    if value < 0_u32 || 0_u32 < value {
        return Err(CanError::ParameterOutOfRange { message_id: 651 });
    }
    self.raw.view_bits_mut::<Lsb0>()[16..48].store_le(value);
    Ok(())
}

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

