use opentelemetry::trace::SpanBuilder;
use serde::ser::{SerializeMap, Serializer as _};
use std::io;
use tracing::{Event, Subscriber};
use tracing_serde::AsSerde;
use tracing_subscriber::{
  fmt::{format::Writer, FmtContext, FormatEvent, FormatFields},
  registry::LookupSpan,
};

pub struct WriteAdaptor<'a> {
  fmt_write: &'a mut dyn std::fmt::Write,
}

impl<'a> WriteAdaptor<'a> {
  pub fn new(fmt_write: &'a mut dyn std::fmt::Write) -> Self {
    Self {
      fmt_write,
    }
  }
}

impl<'a> io::Write for WriteAdaptor<'a> {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    let s = std::str::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    self
      .fmt_write
      .write_str(s)
      .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(s.as_bytes().len())
  }

  fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

pub struct TraceIdFormat;

impl<S, N> FormatEvent<S, N> for TraceIdFormat
where
  S: Subscriber + for<'lookup> LookupSpan<'lookup>,
  N: for<'writer> FormatFields<'writer> + 'static,
{
  fn format_event(
    &self,
    ctx: &FmtContext<'_, S, N>,
    mut writer: Writer<'_>,
    event: &Event<'_>,
  ) -> std::fmt::Result
  where
    S: Subscriber + for<'a> LookupSpan<'a>,
  {
    let meta = event.metadata();

    let mut visit = || {
      let mut serializer = serde_json::Serializer::new(WriteAdaptor::new(&mut writer));

      let mut serializer = serializer.serialize_map(None)?;
      serializer.serialize_entry("level", &meta.level().as_serde())?;

      // let format_field_marker: std::marker::PhantomData<N> = std::marker::PhantomData;

      use tracing_serde::fields::AsMap;
      serializer.serialize_entry("fields", &event.field_map())?;

      serializer.serialize_entry("target", meta.target())?;

      if let Some(span_ref) = ctx.lookup_current() {
        if let Some(builder) = span_ref.extensions().get::<SpanBuilder>() {
          if let Some(trace_id) = builder.trace_id {
            serializer.serialize_entry("trace_id", &format!("{:x?}", trace_id.to_bytes()))?;
          }
        }
      }

      serializer.end()
    };

    visit().map_err(|_| std::fmt::Error)?;
    writeln!(writer)
  }
}
