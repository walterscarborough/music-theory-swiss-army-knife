// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod theory_primitives {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Interval {
  Perfect1 = 0,
  Minor2 = 1,
  Major2 = 2,
  Minor3 = 3,
  Major3 = 4,
  Perfect4 = 5,
  Perfect5 = 6,
  Minor6 = 7,
  Major6 = 8,
  Minor7 = 9,
  Major7 = 10,

}

pub const ENUM_MIN_INTERVAL: u8 = 0;
pub const ENUM_MAX_INTERVAL: u8 = 10;

impl<'a> flatbuffers::Follow<'a> for Interval {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for Interval {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = u8::to_le(self as u8);
    let p = &n as *const u8 as *const Interval;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = u8::from_le(self as u8);
    let p = &n as *const u8 as *const Interval;
    unsafe { *p }
  }
}

impl flatbuffers::Push for Interval {
    type Output = Interval;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<Interval>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
pub const ENUM_VALUES_INTERVAL:[Interval; 11] = [
  Interval::Perfect1,
  Interval::Minor2,
  Interval::Major2,
  Interval::Minor3,
  Interval::Major3,
  Interval::Perfect4,
  Interval::Perfect5,
  Interval::Minor6,
  Interval::Major6,
  Interval::Minor7,
  Interval::Major7
];

#[allow(non_camel_case_types)]
pub const ENUM_NAMES_INTERVAL:[&'static str; 11] = [
    "Perfect1",
    "Minor2",
    "Major2",
    "Minor3",
    "Major3",
    "Perfect4",
    "Perfect5",
    "Minor6",
    "Major6",
    "Minor7",
    "Major7"
];

pub fn enum_name_interval(e: Interval) -> &'static str {
  let index = e as u8;
  ENUM_NAMES_INTERVAL[index as usize]
}

pub enum ScaleOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Scale<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Scale<'a> {
    type Inner = Scale<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Scale<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Scale {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ScaleArgs<'args>) -> flatbuffers::WIPOffset<Scale<'bldr>> {
      let mut builder = ScaleBuilder::new(_fbb);
      if let Some(x) = args.intervals { builder.add_intervals(x); }
      if let Some(x) = args.aliases { builder.add_aliases(x); }
      if let Some(x) = args.name { builder.add_name(x); }
      builder.finish()
    }

    pub const VT_NAME: flatbuffers::VOffsetT = 4;
    pub const VT_ALIASES: flatbuffers::VOffsetT = 6;
    pub const VT_INTERVALS: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Scale::VT_NAME, None)
  }
  #[inline]
  pub fn aliases(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<&'a str>>>>(Scale::VT_ALIASES, None)
  }
  #[inline]
  pub fn intervals(&self) -> Option<flatbuffers::Vector<'a, Interval>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, Interval>>>(Scale::VT_INTERVALS, None)
  }
}

pub struct ScaleArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub aliases: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<&'a  str>>>>,
    pub intervals: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , Interval>>>,
}
impl<'a> Default for ScaleArgs<'a> {
    #[inline]
    fn default() -> Self {
        ScaleArgs {
            name: None,
            aliases: None,
            intervals: None,
        }
    }
}
pub struct ScaleBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ScaleBuilder<'a, 'b> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Scale::VT_NAME, name);
  }
  #[inline]
  pub fn add_aliases(&mut self, aliases: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Scale::VT_ALIASES, aliases);
  }
  #[inline]
  pub fn add_intervals(&mut self, intervals: flatbuffers::WIPOffset<flatbuffers::Vector<'b , Interval>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Scale::VT_INTERVALS, intervals);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ScaleBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ScaleBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Scale<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

}  // pub mod TheoryPrimitives
