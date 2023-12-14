// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum AntiFraudInputOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct AntiFraudInput<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for AntiFraudInput<'a> {
  type Inner = AntiFraudInput<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> AntiFraudInput<'a> {
  pub const VT_INPUTS: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    AntiFraudInput { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args AntiFraudInputArgs<'args>
  ) -> flatbuffers::WIPOffset<AntiFraudInput<'bldr>> {
    let mut builder = AntiFraudInputBuilder::new(_fbb);
    if let Some(x) = args.inputs { builder.add_inputs(x); }
    builder.finish()
  }


  #[inline]
  pub fn inputs(&self) -> Option<flatbuffers::Vector<'a, f64>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, f64>>>(AntiFraudInput::VT_INPUTS, None)}
  }
}

impl flatbuffers::Verifiable for AntiFraudInput<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, f64>>>("inputs", Self::VT_INPUTS, false)?
     .finish();
    Ok(())
  }
}
pub struct AntiFraudInputArgs<'a> {
    pub inputs: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, f64>>>,
}
impl<'a> Default for AntiFraudInputArgs<'a> {
  #[inline]
  fn default() -> Self {
    AntiFraudInputArgs {
      inputs: None,
    }
  }
}

pub struct AntiFraudInputBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> AntiFraudInputBuilder<'a, 'b> {
  #[inline]
  pub fn add_inputs(&mut self, inputs: flatbuffers::WIPOffset<flatbuffers::Vector<'b , f64>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AntiFraudInput::VT_INPUTS, inputs);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> AntiFraudInputBuilder<'a, 'b> {
    let start = _fbb.start_table();
    AntiFraudInputBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<AntiFraudInput<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for AntiFraudInput<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("AntiFraudInput");
      ds.field("inputs", &self.inputs());
      ds.finish()
  }
}
pub enum AntiFraudResponseOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct AntiFraudResponse<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for AntiFraudResponse<'a> {
  type Inner = AntiFraudResponse<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> AntiFraudResponse<'a> {
  pub const VT_RESPONSE: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    AntiFraudResponse { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args AntiFraudResponseArgs<'args>
  ) -> flatbuffers::WIPOffset<AntiFraudResponse<'bldr>> {
    let mut builder = AntiFraudResponseBuilder::new(_fbb);
    if let Some(x) = args.response { builder.add_response(x); }
    builder.finish()
  }


  #[inline]
  pub fn response(&self) -> Option<flatbuffers::Vector<'a, f64>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, f64>>>(AntiFraudResponse::VT_RESPONSE, None)}
  }
}

impl flatbuffers::Verifiable for AntiFraudResponse<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, f64>>>("response", Self::VT_RESPONSE, false)?
     .finish();
    Ok(())
  }
}
pub struct AntiFraudResponseArgs<'a> {
    pub response: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, f64>>>,
}
impl<'a> Default for AntiFraudResponseArgs<'a> {
  #[inline]
  fn default() -> Self {
    AntiFraudResponseArgs {
      response: None,
    }
  }
}

pub struct AntiFraudResponseBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> AntiFraudResponseBuilder<'a, 'b> {
  #[inline]
  pub fn add_response(&mut self, response: flatbuffers::WIPOffset<flatbuffers::Vector<'b , f64>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AntiFraudResponse::VT_RESPONSE, response);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> AntiFraudResponseBuilder<'a, 'b> {
    let start = _fbb.start_table();
    AntiFraudResponseBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<AntiFraudResponse<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for AntiFraudResponse<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("AntiFraudResponse");
      ds.field("response", &self.response());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `AntiFraudInput`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_anti_fraud_input_unchecked`.
pub fn root_as_anti_fraud_input(buf: &[u8]) -> Result<AntiFraudInput, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<AntiFraudInput>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `AntiFraudInput` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_anti_fraud_input_unchecked`.
pub fn size_prefixed_root_as_anti_fraud_input(buf: &[u8]) -> Result<AntiFraudInput, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<AntiFraudInput>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `AntiFraudInput` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_anti_fraud_input_unchecked`.
pub fn root_as_anti_fraud_input_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<AntiFraudInput<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<AntiFraudInput<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `AntiFraudInput` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_anti_fraud_input_unchecked`.
pub fn size_prefixed_root_as_anti_fraud_input_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<AntiFraudInput<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<AntiFraudInput<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a AntiFraudInput and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `AntiFraudInput`.
pub unsafe fn root_as_anti_fraud_input_unchecked(buf: &[u8]) -> AntiFraudInput {
  flatbuffers::root_unchecked::<AntiFraudInput>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed AntiFraudInput and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `AntiFraudInput`.
pub unsafe fn size_prefixed_root_as_anti_fraud_input_unchecked(buf: &[u8]) -> AntiFraudInput {
  flatbuffers::size_prefixed_root_unchecked::<AntiFraudInput>(buf)
}
#[inline]
pub fn finish_anti_fraud_input_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<AntiFraudInput<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_anti_fraud_input_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<AntiFraudInput<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
