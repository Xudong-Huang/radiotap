//! Defines namespaces in the Radiotap capture format.

use failure::ResultExt;
use std::fmt::Debug;

use super::error::ErrorKind;
use super::field::*;
use super::{Oui, Result};

/// A namespace in the Radiotap capture format.
///
/// The default namespace Radiotap implements this trait. To implement a vendor namespace you can
/// use this trait.
pub trait Namespace {
    /// This defines the kinds of fields in a namespace, it is typically implemented as an enum.
    type Kind: NamespaceKind;

    /// Returns the vendor OUI for this namespace. This needs to be unique. The default Radiotap
    /// namespace returns None here.
    fn oui() -> Option<Oui>;

    /// Update this object with the field for the given field kind and relevant data.
    fn update(&mut self, kind: Self::Kind, data: &[u8]) -> Result<()>;
}

/// The kind of field contained in the namespace.
///
/// This defines the kinds of fields in a namespace. This trait is typically implemented as an enum.
/// Each field kind needs to define an field align size and field size.
pub trait NamespaceKind: Sized {
    /// Constructor to create the kind from the bit number.
    fn from_bit(value: u8) -> Result<Self>;

    /// Returns the align value for this kind of field.
    fn align(&self) -> usize;

    /// Returns the size value for this kind of field.
    fn size(&self) -> usize;
}

/// Represents a parsed Radiotap namespace, all fields are optional since not all fields are
/// necessarily present in a Radiotap capture.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Radiotap {
    pub tsft: Option<Tsft>,
    pub flags: Option<Flags>,
    pub rate: Option<Rate>,
    pub channel: Option<Channel>,
    pub fhss: Option<Fhss>,
    pub antenna_signal: Option<AntennaSignal>,
    pub antenna_noise: Option<AntennaNoise>,
    pub lock_quality: Option<LockQuality>,
    pub tx_attenuation: Option<TxAttenuation>,
    pub tx_attenuation_db: Option<TxAttenuationDb>,
    pub tx_power: Option<TxPower>,
    pub antenna: Option<Antenna>,
    pub antenna_signal_db: Option<AntennaSignalDb>,
    pub antenna_noise_db: Option<AntennaNoiseDb>,
    pub rx_flags: Option<RxFlags>,
    pub tx_flags: Option<TxFlags>,
    pub rts_retries: Option<RtsRetries>,
    pub data_retries: Option<DataRetries>,
    pub xchannel: Option<XChannel>,
    pub mcs: Option<Mcs>,
    pub ampdu_status: Option<AmpduStatus>,
    pub vht: Option<Vht>,
    pub timestamp: Option<Timestamp>,

    /// Hints that destructuring should not be exhaustive.
    __non_exhaustive: (),
}

impl Radiotap {
    /// Construct a new unparsed Radiotap namespace.
    pub fn new() -> Self {
        Radiotap {
            ..Default::default()
        }
    }
}

impl Namespace for Radiotap {
    type Kind = RadiotapKind;

    /// Because Radiotap is the default namespace we return None. It is not associated with a
    /// vendor.
    fn oui() -> Option<Oui> {
        None
    }

    /// Updates the Radiotap field with the given Radiotap field kind and raw data.
    fn update(&mut self, kind: Self::Kind, data: &[u8]) -> Result<()> {
        match kind {
            RadiotapKind::Tsft => self.tsft = from_bytes_some(data)?,
            RadiotapKind::Flags => self.flags = from_bytes_some(data)?,
            RadiotapKind::Rate => self.rate = from_bytes_some(data)?,
            RadiotapKind::Channel => self.channel = from_bytes_some(data)?,
            RadiotapKind::Fhss => self.fhss = from_bytes_some(data)?,
            RadiotapKind::AntennaSignal => self.antenna_signal = from_bytes_some(data)?,
            RadiotapKind::AntennaNoise => self.antenna_noise = from_bytes_some(data)?,
            RadiotapKind::LockQuality => self.lock_quality = from_bytes_some(data)?,
            RadiotapKind::TxAttenuation => self.tx_attenuation = from_bytes_some(data)?,
            RadiotapKind::TxAttenuationDb => self.tx_attenuation_db = from_bytes_some(data)?,
            RadiotapKind::TxPower => self.tx_power = from_bytes_some(data)?,
            RadiotapKind::Antenna => self.antenna = from_bytes_some(data)?,
            RadiotapKind::AntennaSignalDb => self.antenna_signal_db = from_bytes_some(data)?,
            RadiotapKind::AntennaNoiseDb => self.antenna_noise_db = from_bytes_some(data)?,
            RadiotapKind::RxFlags => self.rx_flags = from_bytes_some(data)?,
            RadiotapKind::TxFlags => self.tx_flags = from_bytes_some(data)?,
            RadiotapKind::RtsRetries => self.rts_retries = from_bytes_some(data)?,
            RadiotapKind::DataRetries => self.data_retries = from_bytes_some(data)?,
            RadiotapKind::XChannel => self.xchannel = from_bytes_some(data)?,
            RadiotapKind::Mcs => self.mcs = from_bytes_some(data)?,
            RadiotapKind::AmpduStatus => self.ampdu_status = from_bytes_some(data)?,
            RadiotapKind::Vht => self.vht = from_bytes_some(data)?,
            RadiotapKind::Timestamp => self.timestamp = from_bytes_some(data)?,
            _ => {}
        }
        Ok(())
    }
}

/// The Radiotap namespace and all fields it can contain, that we know how to implement.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RadiotapKind {
    Tsft,
    Flags,
    Rate,
    Channel,
    Fhss,
    AntennaSignal,
    AntennaNoise,
    LockQuality,
    TxAttenuation,
    TxAttenuationDb,
    TxPower,
    Antenna,
    AntennaSignalDb,
    AntennaNoiseDb,
    RxFlags,
    TxFlags,
    RtsRetries,
    DataRetries,
    XChannel,
    Mcs,
    AmpduStatus,
    Vht,
    Timestamp,

    /// Hints that destructuring should not be exhaustive.
    #[doc(hidden)]
    __Nonexhaustive,
}

impl NamespaceKind for RadiotapKind {
    fn from_bit(value: u8) -> Result<Self> {
        Ok(match value {
            0 => RadiotapKind::Tsft,
            1 => RadiotapKind::Flags,
            2 => RadiotapKind::Rate,
            3 => RadiotapKind::Channel,
            4 => RadiotapKind::Fhss,
            5 => RadiotapKind::AntennaSignal,
            6 => RadiotapKind::AntennaNoise,
            7 => RadiotapKind::LockQuality,
            8 => RadiotapKind::TxAttenuation,
            9 => RadiotapKind::TxAttenuationDb,
            10 => RadiotapKind::TxPower,
            11 => RadiotapKind::Antenna,
            12 => RadiotapKind::AntennaSignalDb,
            13 => RadiotapKind::AntennaNoiseDb,
            14 => RadiotapKind::RxFlags,
            15 => RadiotapKind::TxFlags,
            16 => RadiotapKind::RtsRetries,
            17 => RadiotapKind::DataRetries,
            18 => RadiotapKind::XChannel,
            19 => RadiotapKind::Mcs,
            20 => RadiotapKind::AmpduStatus,
            21 => RadiotapKind::Vht,
            22 => RadiotapKind::Timestamp,
            v => bail!("invalid Radiotap kind {}", v),
        })
    }

    fn align(&self) -> usize {
        match *self {
            RadiotapKind::Tsft | RadiotapKind::Timestamp => 8,
            RadiotapKind::XChannel | RadiotapKind::AmpduStatus => 4,
            RadiotapKind::Channel
            | RadiotapKind::Fhss
            | RadiotapKind::LockQuality
            | RadiotapKind::TxAttenuation
            | RadiotapKind::TxAttenuationDb
            | RadiotapKind::RxFlags
            | RadiotapKind::TxFlags
            | RadiotapKind::Vht => 2,
            _ => 1,
        }
    }

    fn size(&self) -> usize {
        match *self {
            RadiotapKind::Vht | RadiotapKind::Timestamp => 12,
            RadiotapKind::Tsft | RadiotapKind::AmpduStatus | RadiotapKind::XChannel => 8,
            RadiotapKind::Channel => 4,
            RadiotapKind::Mcs => 3,
            RadiotapKind::Fhss
            | RadiotapKind::LockQuality
            | RadiotapKind::TxAttenuation
            | RadiotapKind::TxAttenuationDb
            | RadiotapKind::RxFlags
            | RadiotapKind::TxFlags => 2,
            _ => 1,
        }
    }
}
