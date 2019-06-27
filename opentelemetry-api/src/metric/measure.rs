use std::borrow::Cow;
use std::marker::PhantomData;

/// Represents a single value recorded for the Measure.
///
/// Measurement *MUST* be treated as immutable short lived object.
/// Instrumentation logic *MUST NOT* hold on to the object and *MUST* only record it once.
pub trait Measurement {}

/// Measure is a contract between the API exposing the raw measurement and an SDK aggregating these
/// values into the Metric.
///
/// Measure is constructed from a Meter, by providing set of Measure identifiers.
pub trait Measure {
    type Measurement: Measurement;
    type Error;

    /// Returns a new `Measurement` for this `Measure`.
    ///
    /// # Panics
    /// * if the type is not {@link Measure.Type#DOUBLE}.
    fn double_measurement<'a>(value: f64) -> Self::Measurement;

    /// Returns a new `Measurement` for this `Measure`.
    ///
    /// # Panics
    /// * if the type is not {@link Measure.Type#LONG}.
    fn long_measurement<'a>(value: i64) -> Self::Measurement;

    fn build(builder: MeasureBuilder<Self>) -> Result<Self, Self::Error>
        where Self: Sized;
}

pub struct MeasureBuilder<'a, M: Measure> {
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub unit: Cow<'a, str>,
    _factory: PhantomData<M>,
}

impl <'a, M: Measure> MeasureBuilder<'a, M> {
    pub fn new<N: Into<Cow<'a, str>>>(name: N) -> Self {
        MeasureBuilder {
            name: name.into(),
            description: "".into(),
            unit: "1".into(),
            _factory: PhantomData,
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
    /// The suggested grammar for a unit is as follows:
    ///
    /// ```ebnf
    ///   Expression = Component { "." Component } {"/" Component };
    ///   Component = [ PREFIX ] UNIT [ Annotation ] | Annotation | "1";
    ///   Annotation = "{" NAME "}" ;
    /// ```
    ///
    /// For example, string `MBy{transmitted}/ms` stands for megabytes per milliseconds, and the
    /// annotation transmitted inside {} is just a comment of the unit.
    ///
    /// Default value is `"1"`.
    pub fn unit<N: Into<Cow<'a, str>>>(mut self, unit: N) -> Self {
        self.unit = unit.into();
        self
    }

    pub fn build(mut self) -> Result<M, M::Error> {
        M::build(self)
    }

}

pub struct NoopMeasurement;
impl Measurement for NoopMeasurement {}

pub struct NoopMeasure;
impl Measure for NoopMeasure {
    type Measurement = NoopMeasurement;
    type Error = ();

    fn double_measurement<'a>(value: f64) -> Self::Measurement {
        unimplemented!()
    }

    fn long_measurement<'a>(value: i64) -> Self::Measurement {
        unimplemented!()
    }

    fn build(builder: MeasureBuilder<Self>) -> Result<Self, Self::Error> where Self: Sized {
        unimplemented!()
    }
}
