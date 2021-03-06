/// Audio sample.
pub type Sample = f32;

/// The channel ordering of audio `Sample`s.
///
/// Multichannel samples are usually interleaved, but other orderings are
/// included if they are needed in the future.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SampleOrder {
  /// Only one channel, no ordering
  Mono,
  /// [L, C, R, L, C, R]
  Interleaved,
  /// [R, C, L, R, C, L]
  Reversed,
  /// [L, L, C, C, R, R]
  Planar
}

/// Converts a type to a `Sample`.
///
/// For integer types, the maximum value will be mapped to a value less than 1.
pub trait ToSample {
  fn to_sample(self) -> Sample;
}

impl ToSample for u8 {
  #[inline]
  fn to_sample(self) -> Sample {
    (self as Sample - 128f32) / 128f32
  }
}

impl ToSample for i8 {
  #[inline]
  fn to_sample(self) -> Sample {
    self as Sample / 128f32
  }
}

impl ToSample for i16 {
  #[inline]
  fn to_sample(self) -> Sample {
    self as Sample / 32_768f32
  }
}

// Headroom is needed for `i32` values due to resolution errors that occur
// during conversions between `f32` and `i32`.
impl ToSample for i32 {
  #[inline]
  fn to_sample(self) -> Sample {
    self as Sample / (2_147_483_648f32 - 128f32)
  }
}

// To be consistent
impl ToSample for f32 {
  #[inline]
  fn to_sample(self) -> Sample {
    if self > 1f32 {
      1f32
    }
    else if self < -1f32 {
      -1f32
    }
    else {
      self
    }
  }
}

impl ToSample for f64 {
  #[inline]
  fn to_sample(self) -> Sample {
    if self > 1f64 {
      1f32
    }
    else if self < -1f64 {
      -1f32
    }
    else {
      self as Sample
    }
  }
}

/// Converts a `Sample` to another type.
///
/// For integer types, mapping the maximum value of a `Sample` are clipped to
/// prevent arithmetic overflow in the result.
pub trait FromSample {
  fn from_sample(sample: Sample) -> Self;
}

impl FromSample for u8 {
  #[inline]
  fn from_sample(sample: Sample) -> Self {
    (i8::from_sample(sample) as i16 + 128) as u8
  }
}

impl FromSample for i8 {
  #[inline]
  fn from_sample(sample: Sample) -> Self {
    let result = sample * 128f32;
    if result > 128f32 - 1f32 {
      i8::max_value()
    }
    else if result < -128f32 {
      i8::min_value()
    }
    else {
      result as i8
    }
  }
}

impl FromSample for i16 {
  #[inline]
  fn from_sample(sample: Sample) -> Self {
    let result = sample * 32_768f32;
    if result > 32_768f32 - 1f32 {
      i16::max_value()
    }
    else if result < -32_768f32 {
      i16::min_value()
    }
    else {
      result as i16
    }
  }
}

// Headroom is needed for `i32` values due to resolution errors that occur
// during conversions between `f32` and `i32`.
impl FromSample for i32 {
  #[inline]
  fn from_sample(sample: Sample) -> Self {
    let result = sample * (2_147_483_648f32 - 128f32);
    if result > (2_147_483_648f32 - 128f32) - 1f32 {
      i32::max_value()
    }
    else if result < -(2_147_483_648f32 - 128f32) {
      i32::min_value()
    }
    else {
      result as i32
    }
  }
}

// To be consistent
impl FromSample for f32 {
  #[inline]
  fn from_sample(sample: Sample) -> Self {
    sample
  }
}

impl FromSample for f64 {
  #[inline]
  fn from_sample(sample: Sample) -> Self {
    sample as f64
  }
}
