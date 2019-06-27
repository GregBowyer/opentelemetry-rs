/*
 * Copyright 2019, OpenTelemetry Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use super::{Metric, TimeSeries, MetricBuilder, LabelValue};

/// Gauge metric, to report instantaneous measurement of a double value. Gauges can go both up and
/// down. The gauges values can be negative.
///
/// <p>Example:
///
/// <pre>{@code
/// class YourClass {
///
///   private static final Meter meter = OpenTelemetry.getMeter();
///   private static final GaugeDouble gauge =
///       meter
///           .gaugeDoubleBuilder("processed_jobs")
///           .setDescription("Processed jobs")
///           .setUnit("1")
///           .setLabelKeys(Collections.singletonList(LabelKey.create("Name", "desc")))
///           .build();
///   // It is recommended to keep a reference of a TimeSeries.
///   private static final GaugeDouble.TimeSeries inboundTimeSeries =
///       gauge.getOrCreateTimeSeries(Collections.singletonList(LabelValue.create("SomeWork")));
///    private static final GaugeDouble.TimeSeries defaultTimeSeries = gauge.getDefaultTimeSeries();
///
///   void doDefault() {
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
pub trait Gauge: Metric {}
pub trait GaugeLong: Gauge {}
pub trait GaugeDouble: Gauge {}

impl_noop_metric!(NoopGaugeLong, NoopTimeSeriesLong);
impl Gauge for NoopGaugeLong {}
impl GaugeLong for NoopGaugeLong {}

impl_noop_metric!(NoopGaugeDouble, NoopTimeSeriesDouble);
impl Gauge for NoopGaugeDouble {}
impl GaugeDouble for NoopGaugeDouble {}

impl_noop_timeseries!(NoopTimeSeriesDouble, f64);
impl_noop_timeseries!(NoopTimeSeriesLong, i64);

