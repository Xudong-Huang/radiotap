//! A parser for the [Radiotap](http://www.radiotap.org/) capture format.
//!
//! # Usage
//!
//! The `Radiotap::from_bytes(&capture)` constructor will parse all present fields into a
//! [Radiotap](struct.Radiotap.html) struct:
//!
//! ```
//! extern crate radiotap;
//! use radiotap::{Radiotap};
//!
//! fn main() {
//!     let capture = [
//!         0, 0, 56, 0, 107, 8, 52, 0, 185, 31, 155, 154, 0, 0, 0, 0, 20, 0, 124, 21, 64, 1, 213,
//!         166, 1, 0, 0, 0, 64, 1, 1, 0, 124, 21, 100, 34, 249, 1, 0, 0, 0, 0, 0, 0, 255, 1, 80,
//!         4, 115, 0, 0, 0, 1, 63, 0, 0
//!     ];
//!
//!     let radiotap = Radiotap::from_bytes(&capture).unwrap();
//!     println!("{:?}", radiotap.vht);
//! }
//!```
//!
//! If you just want to parse a few specific fields from the Radiotap capture you can create an
//! iterator using `CaptureIterator::from_bytes(&capture)`:
//!
//! ```
//! extern crate radiotap;
//! use radiotap::{CaptureIterator, field};
//!
//! fn main() {
//!     let capture = [
//!         0, 0, 56, 0, 107, 8, 52, 0, 185, 31, 155, 154, 0, 0, 0, 0, 20, 0, 124, 21, 64, 1, 213,
//!         166, 1, 0, 0, 0, 64, 1, 1, 0, 124, 21, 100, 34, 249, 1, 0, 0, 0, 0, 0, 0, 255, 1, 80,
//!         4, 115, 0, 0, 0, 1, 63, 0, 0
//!     ];
//!
//!     for element in CaptureIterator::from_bytes(&capture).unwrap() {
//!         match element {
//!             Ok((field::Kind::VHT, data)) => {
//!                 let vht: field::VHT = field::from_bytes(data).unwrap();
//!                 println!("{:?}", vht);
//!             },
//!             _ => {}
//!         }
//!     }
//! }
//! ```

extern crate bitops;
extern crate byteorder;
#[macro_use]
extern crate display_derive;
#[macro_use]
extern crate failure;

pub mod error;
pub mod field;
pub mod ns;

use failure::ResultExt;
use std::collections::HashMap;
use std::io::Cursor;
use std::result;

use error::*;
use field::*;
use ns::*;

/// A return type to use across this crate.
pub type Result<T> = result::Result<T, failure::Error>;

/// The Organizationally Unique Identifier of a vendor.
// #[derive(Eq, Hash, PartialEq)]
pub type Oui = [u8; 3];

/// A trait to align an offset to particular word size, usually 1, 2, 4, or 8.
trait Align {
    /// Aligns the offset to `align` size.
    fn align(&mut self, align: usize);
}

/// We implement align for cursor so that in the Radiotap header we can align the Cursor position
/// to the required word for the field that is about to parsed.
impl<T> Align for Cursor<T> {
    /// Aligns the Cursor position to `align` size.
    fn align(&mut self, align: usize) {
        let p = self.position();
        self.set_position((p + (align as u64) - 1) & !((align as u64) - 1));
    }
}




pub struct CaptureNamespace<'a> {
    namespaces: HashMap<Option<Oui>, Box<dyn Namespace>>,
}





// /// Allows iteration over a Radiotap capture's fields.
// ///
// /// If an unknown vendor namespace is encountered while parsing a Radiotap capture, it is skipped
// /// over. You can give this iterator understanding of a vendor namespace by calling the `vendor()`
// /// function after constructing an iterator.
// ///
// /// **Example**
// ///
// /// ```
// /// CaptureIterator::from_bytes(&frame).vendor(&my_vendor_ns)
// /// ```
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct CaptureIterator<'a> {
//     header: Header,
//     data: &'a [u8],
// }

// impl<'a> CaptureIterator<'a> {
//     /// Creates a new Radiotap capture iterator from an input byte stream. This method just parses
//     /// the Radiotap header and then stops parsing.
//     pub fn from_bytes(input: &'a [u8]) -> Result<Self> {
//         Ok(CaptureIterator::parse(input)?.0)
//     }

//     /// Creates a new Radiotap capture iterator from an input byte stream, and returns the unused
//     /// bytes in the stream. The method just parses the Radiotap header and then stops parsing.
//     pub fn parse(input: &'a [u8]) -> Result<(Self, &'a [u8])> {
//         let header: Header = from_bytes(input).context("invalid Radiotap header")?;
//         let (data, rest) = input.split_at(header.length);
//         Ok((CaptureIterator { header, data }, rest))
//     }
// }

// pub struct CaptureIteratorIntoIter<'a> {
//     /// A cursor over the data that we will move.
//     cursor: Cursor<&'a [u8]>,
//     /// The present words from the Radiotap header field.
//     present: Vec<u32>,
//     /// The current vendor namespace. None if in the default namespace.
//     vendor: Option<VendorNamespace>,
//     /// The current bit index in the present words for the current namespace.
//     bit: u8,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct CaptureField<'a> {
//     oui: Option<Oui>,
//     bit: u8,
//     data: &'a [u8],
// }

// impl<'a> Iterator for CaptureIteratorIntoIter<'a> {
//     type Item = Result<CaptureField<'a>>;

//     fn next(&mut self) -> Option<Self::Item> {
//         // A namespace can use multiple present words.
//         let word_count = self.bit / 32;

//         match self.present.get(word_count.into()) {
//             Some(&word) => {
//                 let bit = self.bit;
//                 let bit_word = self.bit % 32;
//                 // self.bit += 1;

//                 if bit_is_set(word, bit_word) {

//                     // If this bit is set we must move to the default namespace.
//                     if bit_word == 29 {
//                         self.vendor = None;

//                     // If this bit is set we must move to a vendor namespace.
//                     } else if bit_word == 30 {
//                         self.cursor.align(2);  // vendor namespace field is always 2 bytes align
//                         let mut start = self.cursor.position() as usize;
//                         let mut end = start + 6; // Vendor Namespace field is 6 bytes long

//                         match VendorNamespace::from_bytes(&self.cursor.get_ref()[start..end]) {
//                             Ok(vns) => {
//                                 self.vendor = Some(vns)
//                             }
//                         }
//                         let data = ;

//                         from_bytes()
//                     } else {

//                     }

//                     // // If this bit is set it means we are changing namespaces. This bit number is
//                     // // reserved for changing the namespace in all present words.
//                     // if bit_word == 30 {

//                     //     self.bit = 0;
//                     //     self.present.drain(0..word_count as usize);

//                     //     // We are moving to a vendor namespace.
//                     //     if self.vendor.is_none() {
//                     //     // We are moving back to the default namespace.
//                     //     } else {
//                     //         self.vendor = None;
//                     //         self.next()
//                     //     }

//                     // At this point we must know the length and align of the field.
//                     } else {

//                     }
//                 } else {
//                     self.next()
//                 }

//                 // // This means the namespace is changing
//                 // if bit_word == 30 &&{
//                 //     if self.vendor.is_none() {

//                 //     } else {
//                 //         self.vendor = None

//                 //     }

//                 // }

//                 // if bit_is_set(word, word_bit) {

//                 // }
//             }
//             None => None
//         }

// match self.present.first() {
//     Some(&word) => {

//         if !bit_is_set(word.into(), self.current_bit) {
//             self.current_bit += 1;
//             self.next()
//         } else {

//         }

// if self.current_bit == 30 && bit_is_set(word.into(), self.current_bit) {

// } else {

// }

// if bit_is_set(word.into(), self.current_bit) {

// } else {

// }
// }
// None => None
// }
// }

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.present.pop() {
//             Some(mut kind) => {
//                 // Align the cursor to the current field's needed alignment.
//                 self.cursor.align(kind.align());

//                 let mut start = self.cursor.position() as usize;
//                 let mut end = start + kind.size();

//                 // The header lied about how long the body was
//                 if end > self.cursor.get_ref().len() {
//                     Some(Err(ErrorKind::IncompleteError))
//                 } else {
//                     // Switching to a vendor namespace, and we don't know how to handle
//                     // so we just return the entire vendor namespace section
//                     if kind == Kind::VendorNamespace(None) {
//                         match VendorNamespace::from_bytes(&self.cursor.get_ref()[start..end]) {
//                             Ok(vns) => {
//                                 start += kind.size();
//                                 end += vns.skip_length as usize;
//                                 kind = Kind::VendorNamespace(Some(vns));
//                             }
//                             Err(e) => return Some(Err(e)),
//                         }
//                     }
//                     let data = &self.cursor.get_ref()[start..end];
//                     self.cursor.set_position(end as u64);
//                     Some(Ok((kind, data)))
//                 }
//             }
//             None => None,
//         }
//     }
// }

// #[derive(Clone, Debug)]
// pub struct Context<'a> {
//     /// The default Radiotap namespace to populate with the parsed information.
//     default: Radiotap,
//     /// Any vendor namespaces to populate with parsed information.
//     vendors: Vec<&'a Namespace>,
// }

// impl<'a> Context<'a> {

//     /// Construct a new Context object.
//     pub fn new() -> Self {
//         Context {
//             default: Radiotap::new(),
//             vendors: Vec::new(),
//         }
//     }

//     /// Add a vendor namespace to the context.
//     pub fn vendor<N: Namespace>(mut self, ns: &'a N) -> Self {
//         self.vendors.push(ns);
//         self
//     }

//     /// Parse input.
//     pub fn parse(mut self, input: &[u8]) -> Result<(Capture, &[u8])> {

//     }
// }

// #[derive(Clone, Debug)]
// pub struct Capture {
//     header: Header
// }

// #[derive(Debug)]
// pub struct Capture {
//     default: Radiotap,
//     vendors: Vec<Box<Namespace>>,
// }

// impl Capture {
//     pub fn new() -> Self {
//         Capture {
//             default: Radiotap::new(),
//             vendors: Vec::new(),
//         }
//     }

//     pub fn vendor<N: Namespace + 'static>(mut self, ns: N) -> Self {
//         self.vendors.push(Box::new(ns));
//         self
//     }

//     pub fn parse(mut self, input: &[u8]) -> Result<(Self, &[u8])> {
//         let header: Header = from_bytes(input)?;
//         let (data, rest) = input.split_at(header.length);
//         Ok((self, rest))
//     }
// }
// #[derive(Debug)]
// pub struct Capture<'a> {
//     header: Header,
//     data: &'a [u8],
// }

// impl<'a> CaptureIterator<'a> {
//     pub fn from_bytes(input: &'a [u8]) -> Result<CaptureIterator<'a>> {
//         Ok(CaptureIterator::parse(input)?.0)
//     }

//     pub fn parse(input: &'a [u8]) -> Result<(CaptureIterator<'a>, &'a [u8])> {
//         let header: Header = from_bytes(input).context("invalid Radiotap header")?;
//         let (data, rest) = input.split_at(header.length);
//         Ok((CaptureIterator { header, data }, rest))
//     }
// }

// pub struct CaptureIteratorIntoIter<'a> {
//     cursor: Cursor<&'a [u8]>,
//     present: &Vec<u32>,

//     default: &'a Radiotap,
//     namespaces: HashMap<Oui, Box<Namespace>>,

//     present_bit_index: u8,
//     present_word_index: u8,
// }

// impl<'a> Iterator for CaptureIteratorIntoIter<'a> {
//     type Item = Result<(u8, &'a [u8])>;

//     fn next(&mut self) -> Option<Self::Item> {

//     }
// }

// impl<'a> Iterator for CaptureIteratorIntoIter<'a> {
//     type Item = Result<(Kind, &'a [u8])>;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.present.pop() {
//             Some(mut kind) => {
//                 // Align the cursor to the current field's needed alignment.
//                 self.cursor.align(kind.align());

//                 let mut start = self.cursor.position() as usize;
//                 let mut end = start + kind.size();

//                 // The header lied about how long the body was
//                 if end > self.cursor.get_ref().len() {
//                     Some(Err(ErrorKind::IncompleteError))
//                 } else {
//                     // Switching to a vendor namespace, and we don't know how to handle
//                     // so we just return the entire vendor namespace section
//                     if kind == Kind::VendorNamespace(None) {
//                         match VendorNamespace::from_bytes(&self.cursor.get_ref()[start..end]) {
//                             Ok(vns) => {
//                                 start += kind.size();
//                                 end += vns.skip_length as usize;
//                                 kind = Kind::VendorNamespace(Some(vns));
//                             }
//                             Err(e) => return Some(Err(e)),
//                         }
//                     }
//                     let data = &self.cursor.get_ref()[start..end];
//                     self.cursor.set_position(end as u64);
//                     Some(Ok((kind, data)))
//                 }
//             }
//             None => None,
//         }
//     }
// }

// impl<'a> IntoIterator for &'a CaptureIterator<'a> {
//     type Item = Result<(u8, &'a [u8])>;
//     type IntoIter = CaptureIteratorIntoIter<'a>;

// fn into_iter(self) -> Self::IntoIter {
//     // let present = self.header.present.iter().rev().cloned().collect();
//     // let mut cursor = Cursor::new(self.data);
//     // cursor.set_position(self.header.size as u64);
//     // CaptureIteratorIntoIter { present, cursor }
// }
// }

// impl<'a> CaptureIterator<'a> {
//     pub fn from_bytes(input: &'a [u8]) -> Result<CaptureIterator<'a>> {
//         Ok(CaptureIterator::parse(input)?.0)
//     }

//     pub fn parse(input: &'a [u8]) -> Result<(CaptureIterator<'a>, &'a [u8])> {
//         let header: Header = from_bytes(input)?;
//         let (data, rest) = input.split_at(header.length);
//         Ok((CaptureIterator { header, data }, rest))
//     }
// }

// /// An iterator over Radiotap capture fields.
// #[doc(hidden)]
// #[derive(Debug, Clone)]
// pub struct CaptureIteratorIntoIter<'a> {
//     index: u8,
//     present: Vec<u32>,
//     cursor: Cursor<&'a [u8]>,
// }

// impl<'a> IntoIterator for &'a CaptureIterator<'a> {
//     type Item = Result<(Kind, &'a [u8])>;
//     type IntoIter = CaptureIteratorIntoIter<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         let present = self.header.present.iter().rev().cloned().collect();
//         let mut cursor = Cursor::new(self.data);
//         cursor.set_position(self.header.size as u64);
//         CaptureIteratorIntoIter { present, cursor }
//     }
// }

// impl<'a> IntoIterator for CaptureIterator<'a> {
//     type Item = Result<(Kind, &'a [u8])>;
//     type IntoIter = CaptureIteratorIntoIter<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         let present = self.header.present.iter().rev().cloned().collect();
//         let mut cursor = Cursor::new(self.data);
//         cursor.set_position(self.header.size as u64);
//         CaptureIteratorIntoIter { present, cursor }
//     }
// }

// impl<'a> Iterator for CaptureIteratorIntoIter<'a> {
//     type Item = Result<(Kind, &'a [u8])>;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.present.pop() {
//             Some(mut kind) => {
//                 // Align the cursor to the current field's needed alignment.
//                 self.cursor.align(kind.align());

//                 let mut start = self.cursor.position() as usize;
//                 let mut end = start + kind.size();

//                 // The header lied about how long the body was
//                 if end > self.cursor.get_ref().len() {
//                     Some(Err(ErrorKind::IncompleteError))
//                 } else {
//                     // Switching to a vendor namespace, and we don't know how to handle
//                     // so we just return the entire vendor namespace section
//                     if kind == Kind::VendorNamespace(None) {
//                         match VendorNamespace::from_bytes(&self.cursor.get_ref()[start..end]) {
//                             Ok(vns) => {
//                                 start += kind.size();
//                                 end += vns.skip_length as usize;
//                                 kind = Kind::VendorNamespace(Some(vns));
//                             }
//                             Err(e) => return Some(Err(e)),
//                         }
//                     }
//                     let data = &self.cursor.get_ref()[start..end];
//                     self.cursor.set_position(end as u64);
//                     Some(Ok((kind, data)))
//                 }
//             }
//             None => None,
//         }
//     }
// }

// impl Default for Header {
//     fn default() -> Header {
//         Header {
//             version: 0,
//             length: 8,
//             present: Vec::new(),
//             size: 8,
//         }
//     }
// }

// impl Radiotap {
//     /// Returns the parsed [Radiotap](struct.Radiotap.html) from an input byte array.
//     pub fn from_bytes(input: &[u8]) -> Result<Radiotap> {
//         Ok(Radiotap::parse(input)?.0)
//     }

//     /// Returns the parsed [Radiotap](struct.Radiotap.html) and remaining data from an input byte
//     /// array.
//     pub fn parse(input: &[u8]) -> Result<(Radiotap, &[u8])> {
//         let (iterator, rest) = CaptureIterator::parse(input)?;

//         let mut radiotap = Radiotap {
//             header: iterator.header.clone(),
//             ..Default::default()
//         };

//         for result in &iterator {
//             let (field_kind, data) = result?;

//             match field_kind {
//                 RadiotapKind::TSFT => radiotap.tsft = from_bytes_some(data)?,
//                 RadiotapKind::Flags => radiotap.flags = from_bytes_some(data)?,
//                 RadiotapKind::Rate => radiotap.rate = from_bytes_some(data)?,
//                 RadiotapKind::Channel => radiotap.channel = from_bytes_some(data)?,
//                 RadiotapKind::FHSS => radiotap.fhss = from_bytes_some(data)?,
//                 RadiotapKind::AntennaSignal => radiotap.antenna_signal = from_bytes_some(data)?,
//                 RadiotapKind::AntennaNoise => radiotap.antenna_noise = from_bytes_some(data)?,
//                 RadiotapKind::LockQuality => radiotap.lock_quality = from_bytes_some(data)?,
//                 RadiotapKind::TxAttenuation => radiotap.tx_attenuation = from_bytes_some(data)?,
//                 RadiotapKind::TxAttenuationDb => {
//                     radiotap.tx_attenuation_db = from_bytes_some(data)?
//                 }
//                 RadiotapKind::TxPower => radiotap.tx_power = from_bytes_some(data)?,
//                 RadiotapKind::Antenna => radiotap.antenna = from_bytes_some(data)?,
//                 RadiotapKind::AntennaSignalDb => {
//                     radiotap.antenna_signal_db = from_bytes_some(data)?
//                 }
//                 RadiotapKind::AntennaNoiseDb => radiotap.antenna_noise_db = from_bytes_some(data)?,
//                 RadiotapKind::RxFlags => radiotap.rx_flags = from_bytes_some(data)?,
//                 RadiotapKind::TxFlags => radiotap.tx_flags = from_bytes_some(data)?,
//                 RadiotapKind::RTSRetries => radiotap.rts_retries = from_bytes_some(data)?,
//                 RadiotapKind::DataRetries => radiotap.data_retries = from_bytes_some(data)?,
//                 RadiotapKind::XChannel => radiotap.xchannel = from_bytes_some(data)?,
//                 RadiotapKind::MCS => radiotap.mcs = from_bytes_some(data)?,
//                 RadiotapKind::AMPDUStatus => radiotap.ampdu_status = from_bytes_some(data)?,
//                 RadiotapKind::VHT => radiotap.vht = from_bytes_some(data)?,
//                 RadiotapKind::Timestamp => radiotap.timestamp = from_bytes_some(data)?,
//                 _ => {}
//             }
//         }

//         Ok((radiotap, rest))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_vendor() {
        let frame = [
            0, 0, 39, 0, 46, 72, 0, 192, 0, 0, 0, 128, 0, 0, 0, 160, 4, 0, 0, 0, 16, 2, 158, 9,
            160, 0, 227, 5, 0, 0, 255, 255, 255, 255, 2, 0, 222, 173, 4,
        ];

        assert_eq!(
            Radiotap::from_bytes(&frame).unwrap().rate.unwrap(),
            Rate { value: 2.0 }
        );
    }

    #[test]
    fn bad_version() {
        let frame = [
            1, 0, 39, 0, 46, 72, 0, 192, 0, 0, 0, 128, 0, 0, 0, 160, 4, 0, 0, 0, 16, 2, 158, 9,
            160, 0, 227, 5, 0, 0, 255, 255, 255, 255, 2, 0, 222, 173, 4,
        ];

        match Radiotap::from_bytes(&frame).unwrap_err() {
            ErrorKind::UnsupportedVersion => {}
            e => panic!("Error not UnsupportedVersion: {:?}", e),
        };
    }

    #[test]
    fn bad_header_length() {
        let frame = [
            0, 0, 40, 0, 46, 72, 0, 192, 0, 0, 0, 128, 0, 0, 0, 160, 4, 0, 0, 0, 16, 2, 158, 9,
            160, 0, 227, 5, 0, 0, 255, 255, 255, 255, 2, 0, 222, 173, 4,
        ];

        match Radiotap::from_bytes(&frame).unwrap_err() {
            ErrorKind::InvalidLength => {}
            e => panic!("Error not InvalidLength: {:?}", e),
        };
    }

    #[test]
    fn bad_actual_length() {
        let frame = [
            0, 0, 39, 0, 47, 72, 0, 192, 0, 0, 0, 128, 0, 0, 0, 160, 4, 0, 0, 0, 16, 2, 158, 9,
            160, 0, 227, 5, 0, 0, 255, 255, 255, 255, 2, 0, 222, 173, 4,
        ];

        match Radiotap::from_bytes(&frame).unwrap_err() {
            ErrorKind::IncompleteError => {}
            e => panic!("Error not IncompleteError: {:?}", e),
        };
    }

    #[test]
    fn bad_vendor() {
        let frame = [
            0, 0, 34, 0, 46, 72, 0, 192, 0, 0, 0, 128, 0, 0, 0, 160, 4, 0, 0, 0, 16, 2, 158, 9,
            160, 0, 227, 5, 0, 0, 255, 255, 255, 255,
        ];

        match Radiotap::from_bytes(&frame).unwrap_err() {
            ErrorKind::IncompleteError => {}
            e => panic!("Error not IncompleteError: {:?}", e),
        };
    }
}
