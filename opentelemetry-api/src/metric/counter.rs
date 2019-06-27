use super::{Metric, TimeSeries, MetricBuilder, LabelValue};

/// Counter metric, to report instantaneous measurement of a double value.
/// Cumulative values can go up or stay the same, but can never go down.
/// Cumulative values cannot be negative.
///
/// # Example:
///
/// <pre>{@code
/// class YourClass {
///
///   private static final Meter meter = OpenTelemetry.getMeter();
///   private static final CounterDouble counter =
///       meter.
///           .counterDoubleBuilder("processed_jobs")
///           .setDescription("Processed jobs")
///           .setUnit("1")
///           .setLabelKeys(Collections.singletonList(LabelKey.create("Name", "desc")))
///           .build();
///   // It is recommended to keep a reference of a TimeSeries.
///   private static final CounterDouble.TimeSeries inboundTimeSeries =
///       counter.getOrCreateTimeSeries(Collections.singletonList(LabelValue.create("SomeWork")));
///   private static final CounterDouble.TimeSeries defaultTimeSeries =
///       counter.getDefaultTimeSeries();
///
///   void doDefaultWork() {
///      // Your code here.
///      defaultTimeSeries.add(10);
///   }
///
///   void doSomeWork() {
///      // Your code here.
///      inboundTimeSeries.set(15);
///   }
///
/// }
/// }</pre>
pub trait Counter: Metric {}
pub trait CounterLong: Counter {}
pub trait CounterDouble: Counter {}

impl_noop_metric!(NoopCounterLong, NoopTimeSeriesLong);
impl Counter for NoopCounterLong {}
impl CounterLong for NoopCounterLong {}

impl_noop_metric!(NoopCounterDouble, NoopTimeSeriesDouble);
impl Counter for NoopCounterDouble {}
impl CounterDouble for NoopCounterDouble {}

impl_noop_timeseries!(NoopTimeSeriesDouble, f64);
impl_noop_timeseries!(NoopTimeSeriesLong, i64);
