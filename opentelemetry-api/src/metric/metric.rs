use std::{
    collections::HashMap,
    borrow::Cow,
    marker::PhantomData,
};

use crate::Resource;
use super::{LabelValue, LabelKey};

pub trait Metric {
    type Error;
    type TS: TimeSeries;

    /// Creates a `TimeSeries` and returns a `TimeSeries` if the specified `labelValues` is not
    /// already associated with this gauge, else returns an existing `TimeSeries`.
    ///
    /// It is recommended to keep a reference to the `TimeSeries` instead of always calling this
    /// method for every operations.
    ///
    /// # Params
    /// * `labelValues` the list of label values.
    ///    The number of label values must be the same to that of the label keys passed to
    ///    `GaugeDouble.Builder.set_label_keys`
    ///
    /// # Panics
    /// * if the number of `labelValues`s are not equal to the label keys.
    fn timeseries(&self, label_values: Vec<LabelValue>) -> Self::TS;

    /// Returns a `TimeSeries` for a metric with all labels not set (default label value).
    fn default_timeseries(&self) -> Self::TS;

    /// Sets a callback that gets executed every time before exporting this metric.
    ///
    /// Evaluation is deferred until needed, if this `Metric` is not exported then it will never
    /// be called.
    //fn set_callback<F>(metric_updater: F) where F: FnOnce(&Metric) -> &Metric;

    /// Removes the `TimeSeries` from the metric, if it is present.
    ///
    /// i.e. references to previous `TimeSeries` are invalid (not part of the metric).
    fn remove_timeseries(&self, label_values: Vec<LabelValue>);

    /// Removes all `TimeSeries` from the metric
    ///
    /// i.e. references to all previous `TimeSeries` are invalid (not part of the metric).
    fn clear();

    /// Builds instances of metrics from a given MetricBuilder
    fn build(mb: MetricBuilder<Self>) -> Result<Self, Self::Error>
        where Self: Sized;
}

pub struct MetricBuilder<'a, M: Metric> {
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub unit: Cow<'a, str>,
    pub label_keys: Vec<LabelKey<'a>>,
    pub constant_labels: HashMap<LabelKey<'a>, LabelValue<'a>>,
    pub component: Option<Cow<'a, str>>,
    pub resource: Option<Resource<'a>>,

    _factory: PhantomData<M>,
}

impl <'a, M: Metric> MetricBuilder<'a, M> {

    pub fn new<N: Into<Cow<'a, str>>>(name: N) -> Self {
        MetricBuilder {
            name: name.into(),
            description: "".into(),
            unit: "1".into(),
            label_keys: Vec::default(),
            constant_labels: HashMap::default(),
            component: None,
            resource: None,
            _factory: PhantomData
        }
    }

    /// Sets the description of the `Metric`.
    ///
    /// Default value is `""`.
    pub fn description<N: Into<Cow<'a, str>>>(mut self, description: N) -> Self {
        self.description = description.into();
        self
    }

    /// Sets the unit of the `Metric`.
    ///
    /// Default value is `"1"`.
    pub fn unit<N: Into<Cow<'a, str>>>(mut self, unit: N) -> Self {
        self.unit = unit.into();
        self
    }

    /// Sets the list of label keys for the Metric.
    ///
    /// Default value is `[]`
    pub fn label_keys(mut self, label_keys: Vec<LabelKey<'a>>) -> Self {
        self.label_keys = label_keys;
        self
    }

    /// Sets the map of constant labels (they will be added to all the TimeSeries) for the Metric.
    ///
    /// Default value is `{}`
    pub fn constant_labels(mut self, constant_labels: HashMap<LabelKey<'a>, LabelValue<'a>>) -> Self {
        self.constant_labels = constant_labels;
        self
    }

    /// Sets the name of the component that reports this `Metric`.
    ///
    /// The final name of the reported metric will be `component + "_" + name` if the
    /// component is not empty.
    ///
    /// It is recommended to always set a component name for all the metrics, because some
    /// implementations may filter based on the component.
    pub fn component<C: Into<Cow<'a, str>>>(mut self, component: C) -> Self {
        self.component = Some(component.into());
        self
    }

    /// Sets the `Resource` associated with this `Metric`.
    ///
    /// This should be set only when reporting out-of-band metrics, otherwise the implementation
    /// will set the `Resource` for in-process metrics (or user can do that when initialize the
    /// `Meter`).
    pub fn resource(mut self, resource: Resource<'a>) -> Self {
        self.resource = Some(resource);
        self
    }

    /// Builds and returns a metric with the desired options.
    pub fn build(self) -> Result<M, M::Error> {
        M::build(self)
    }
}

pub trait TimeSeries: Default {
    type V;

    /// Adds the given value to the current value. The values cannot be negative.
    fn add(&self, delta: Self::V);

    /// Sets the given value. The value must be larger than the current recorded value.
    ///
    /// In general should be used in combination with `setCallback(...)` where the
    /// recorded value is guaranteed to be monotonically increasing.
    fn set(&self, val: Self::V);
}

