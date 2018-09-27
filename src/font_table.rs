use quick_xml::events::BytesStart;
use std::borrow::Cow;

use errors::{Error, Result};
use schema::{SCHEMA_MAIN, SCHEMA_RELATIONSHIPS};

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:fonts")]
#[xml(extend_attrs = "font_table_extend_attrs")]
pub struct FontTable<'a> {
  #[xml(child)]
  #[xml(tag = "w:font")]
  fonts: Vec<Font<'a>>,
}

fn font_table_extend_attrs(_: &FontTable, start: &mut BytesStart) {
  start.push_attribute(("xmlns:w", SCHEMA_MAIN));
  start.push_attribute(("xmlns:r", SCHEMA_RELATIONSHIPS));
}

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:font")]
pub struct Font<'a> {
  #[xml(attr = "w:name")]
  name: Cow<'a, str>,
  #[xml(flatten_empty)]
  #[xml(tag = "w:charset")]
  #[xml(attr = "w:val")]
  charset: Option<Cow<'a, str>>,
  #[xml(flatten_empty)]
  #[xml(tag = "w:family")]
  #[xml(attr = "w:val")]
  family: Option<Cow<'a, str>>,
  #[xml(flatten_empty)]
  #[xml(tag = "w:pitch")]
  #[xml(attr = "w:val")]
  pitch: Option<Cow<'a, str>>,
}