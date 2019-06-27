use std::borrow::Cow;
use crate::metric::{
    MetricBuilder,
    CounterDouble, CounterLong, GaugeDouble, GaugeLong,
    counter, gauge, measure,
    measure::{Measure, MeasureBuilder},
};

/// Allows users to record measurements (metrics).
///
/// There are two ways to record measurements:
///
/// * Record raw measurements
///   Defer defining the aggregation and the labels for the exported `Metric`.
///   This should be used in libraries like gRPC to record measurements like "server_latency" or
///   "received_bytes".
///
/// * Record pre-defined aggregation data (or already aggregated data).
///   This should be used to report cpu/memory usage, or simple metrics like "queue_length".
///
/// # Examples for raw measurement:
///
/// ```
///
/// ```
///
/// <pre>{@code
/// class MyClass {
///   private static final Meter meter = Metrics.getMeter();
///   private static final Measure cacheHit = meter.measureBuilder("cache_hit").build();
///
///   Response serverHandler(Request request) {
///     if (inCache(request)) {
///       meter.record(Collections.singletonList(cacheHit.createMeasurement(1)));
///       return fromCache(request);
///     }
///     ...  // do other work
///   }
///
/// }
/// }</pre>
///
/// # Examples for already aggregated metrics:
///
/// ```
/// ```
///
/// <pre>{@code
/// class YourClass {
///   private static final Meter meter = Metrics.getMeter();
///   private static final CounterLong collectionMetric =
///       meter
///           .counterLongBuilder("collection")
///           .setDescription("Time spent in a given JVM garbage collector in milliseconds.")
///           .setUnit("ms")
///           .setLabelKeys(Collections.singletonList(GC))
///           .build();
///
///   public final void exportGarbageCollectorMetrics {
///     collectionMetric.setCallback(
///         new Runnable() {
///           &commat;Override
///           public void run() {
///             for (GarbageCollectorMXBean gc : ManagementFactory.getGarbageCollectorMXBeans()) {
///               LabelValue gcName = LabelValue.create(gc.getName());
///               collectionMetric
///                   .getOrCreateTimeSeries(Collections.singletonList(gcName))
///                   .set(gc.getCollectionTime());
///             }
///           }
///         });
///   }
/// }
/// }</pre>
///
/// # Example usage for simple pre-defined aggregation metrics:
///
/// ```
/// ```
///
/// <pre>{@code
/// class YourClass {
///   private static final Meter meter = Metrics.getMeter();
///   private static final List<LabelKey> keys = Arrays.asList(LabelKey.create("Name", "desc"));
///   private static final List<LabelValue> values = Arrays.asList(LabelValue.create("Inbound"));
///   private static final GaugeDouble gauge = metricRegistry.gaugeLongBuilder(
///       "queue_size", "Pending jobs", "1", labelKeys);
///
///   // It is recommended to keep a reference of a TimeSeries.
///   GaugeDouble.TimeSeries inboundTimeSeries = gauge.getOrCreateTimeSeries(labelValues);
///
///   void doAddElement() {
///      // Your code here.
///      inboundTimeSeries.add(1);
///   }
///
///   void doRemoveElement() {
///      inboundTimeSeries.add(-1);
///      // Your code here.
///   }
///
/// }
/// }</pre
pub trait Meter {
    type CL: CounterLong;
    type CD: CounterDouble;
    type GL: GaugeLong;
    type GD: GaugeDouble;
    type Measure: Measure;

    /// Returns a builder for a `GaugeLong` to be added to the registry.
    ///
    /// # Panics
    /// * if different metric with the same name already registered.
    fn gauge_long<'a, N: Into<Cow<'a, str>>>(name: N) -> MetricBuilder<'a, Self::GL> {
        MetricBuilder::new(name)
    }

    /// Returns a builder for a `GaugeDouble` to be added to the registry.
    ///
    /// # Panics
    /// * if different metric with the same name already registered.
    fn gauge_double<'a, N: Into<Cow<'a, str>>>(name: N) -> MetricBuilder<'a, Self::GD> {
        MetricBuilder::new(name)
    }

    /// Returns a builder for a `CounterDouble` to be added to the registry.
    ///
    /// # Panics
    /// * if different metric with the same name already registered.
    fn counter_double<'a, N: Into<Cow<'a, str>>>(&mut self, name: N) -> MetricBuilder<'a, Self::CD> {
        MetricBuilder::new(name)
    }

    /// Returns a builder for a `CounterLong` to be added to the registry.
    ///
    /// # Panics
    /// * if different metric with the same name already registered.
    fn counter_long<'a, N: Into<Cow<'a, str>>>(&mut self, name: N) -> MetricBuilder<'a, Self::CL> {
        MetricBuilder::new(name)
    }

    /// Returns a new builder for a `Measure`.
    fn measure<'a, N: Into<Cow<'a, str>>>(&mut self, name: N) -> MeasureBuilder<'a, Self::Measure> {
        MeasureBuilder::new(name)
    }

    /// Records all given measurements, with the current
    /// `opentelemetry.distributedcontext.DistributedContextManager::current_context()`
    fn record<I>(&mut self, measurements: I)
        where I: IntoIterator<Item=<<Self as Meter>::Measure as Measure>::Measurement>;

    /*

    /// Records all given measurements, with an explicit `DistributedContext`.
    fn record_with_context<I>(&mut self, measurements: I, dist_context: &DistributedContext)
        where I: IntoIterator<Item=<<Self as Meter>::Measure as Measure>::Measurement>;

    /// Records all given measurements, with an explicit `DistributedContext`.
    /// These measurements are associated with the given `SpanContext`.
    // TODO: Avoid tracing dependency and accept Attachments as in OpenCensus.
    fn record_with_context_and_span<I>(&mut self, measurements: I, dist_context: &DistributedContext,
                                       span_context: &SpanContext)
        where I: IntoIterator<Item=<<Self as Meter>::Measure as Measure>::Measurement>;
    */
}

pub struct DefaultMeter;

impl Meter for DefaultMeter {
    type CL = counter::NoopCounterLong;
    type CD = counter::NoopCounterDouble;
    type GL = gauge::NoopGaugeLong;
    type GD = gauge::NoopGaugeDouble;
    type Measure = measure::NoopMeasure;

    fn record<I>(&mut self, measurements: I) where I: IntoIterator<Item=measure::NoopMeasurement> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noops() {
        let mut m = DefaultMeter{};
        let counter = m.counter_long("test").build().unwrap();
    }
}