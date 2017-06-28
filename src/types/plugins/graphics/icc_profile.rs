//! ICC profile that can be embedded into a PDF

extern crate lopdf;

use *;

/// Type of the icc profile
#[derive(Debug, Clone, PartialEq)]
pub enum IccProfileType {
    Cmyk,
    Rgb,
    Grayscale,
}

/// Icc profile
#[derive(Debug, Clone, PartialEq)]
pub struct IccProfile {
    /// Binary Icc profile
    icc: Vec<u8>,
    /// CMYK or RGB or LAB icc profile?
    icc_type: IccProfileType,
    /// Does the ICC profile have an "Alternate" version or not?
    pub has_alternate: bool,
    /// Does the ICC profile have an "Range" dictionary
    /// Really not sure why this is needed, but this is needed on the documents Info dictionary
    pub has_range: bool,
}

impl IccProfile {
    /// Creates a new Icc Profile
    pub fn new(icc: Vec<u8>, icc_type: IccProfileType)
    -> Self 
    {
        Self { 
            icc: icc, 
            icc_type: icc_type,
            has_alternate: true,
            has_range: false,
        }
    }

    /// Does the ICC profile have an alternate version (such as "DeviceCMYk")?
    #[inline]
    pub fn with_alternate_profile(mut self, has_alternate: bool)
    -> Self 
    {
        self.has_alternate = has_alternate;
        self
    }

    /// Does the ICC profile have an "Range" dictionary?
    #[inline]
    pub fn with_range(mut self, has_range: bool)
    -> Self 
    {
        self.has_range = has_range;
        self
    }

}

impl IntoPdfObject for IccProfile {
    fn into_obj(self: Box<Self>)
    -> Vec<lopdf::Object>
    {
        use lopdf::{Dictionary as LoDictionary, 
                    Stream as LoStream};
        use lopdf::Object::*;
        use std::iter::FromIterator;

        let (num_icc_fields, alternate) = match self.icc_type {
            IccProfileType::Cmyk => (4, "DeviceCMYK"),
            IccProfileType::Rgb => (3, "DeviceRGB"),
            IccProfileType::Grayscale => (1, "DeviceGray"),
        };

        let mut stream_dict = LoDictionary::from_iter(vec![
                ("N", Integer(num_icc_fields)).into(),
                ("Length", Integer(self.icc.len() as i64).into())]);

        if self.has_alternate {
            stream_dict.set("Alternate", Name(alternate.into()));
        }

        if self.has_range {
            stream_dict.set("Range", Array(vec![
                                        Real(0.0),
                                        Real(1.0),
                                        Real(0.0),
                                        Real(1.0),
                                        Real(0.0),
                                        Real(1.0),
                                        Real(0.0),
                                        Real(1.0)]));
        }

        let stream = LoStream::new(stream_dict, self.icc);

        vec![Stream(stream)]
    }
}