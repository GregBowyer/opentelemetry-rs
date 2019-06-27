use std::borrow::Cow;

#[macro_use]
mod macros;

pub mod metric;
pub use metric::{Metric, MetricBuilder, TimeSeries};

pub mod meter;
pub use meter::Meter;

pub mod measure;
pub use measure::{Measurement, Measure, MeasureBuilder};

pub mod gauge;
pub use gauge::{Gauge, GaugeDouble, GaugeLong};

pub mod counter;
pub use counter::{Counter, CounterDouble, CounterLong};

/// Defines a label key associated with a metric descriptor.
#[derive(Hash, Eq, PartialEq)]
pub struct LabelKey<'a> {
    /// The key for the label.
    pub key: Cow<'a, str>,

    /// A human-readable description of what this label key represents.
    pub description: Cow<'a, str>,
}

pub struct LabelValue<'a> {
    /// The value for the label.
    pub value: Cow<'a, str>,

    /// If false the value field is ignored and considered not set.
    /// This is used to differentiate a missing label from an empty string.
    pub has_value: bool,
}

#[cfg(test)]
mod test {

    #[test]
    fn test_x() {

    }

}
