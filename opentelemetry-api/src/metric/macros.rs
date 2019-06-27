/// Macro to make it easy to generate Noop metrics
macro_rules! impl_noop_metric {
    ($name:ident, $ts:ident) => (
        pub struct $name;
        impl Metric for $name {
            type Error = ();
            type TS = $ts;

            fn timeseries(&self, _label_values: Vec<LabelValue>) -> Self::TS {
                Self::TS::default()
            }

            fn default_timeseries(&self) -> Self::TS {
                Self::TS::default()
            }

            fn remove_timeseries(&self, _label_values: Vec<LabelValue>) {
                unimplemented!()
            }

            fn clear() {
                unimplemented!()
            }

            fn build(_mb: MetricBuilder<Self>) -> Result<Self, ()> {
                Ok($name{})
            }
        }
    );
}

/// Macro to make it easier to generate Noop timeseries
macro_rules! impl_noop_timeseries {
    ($name: ident, $val: ty) => (
        #[derive(Default)]
        pub struct $name;
        impl TimeSeries for $name {
            type V = $val;
            fn add(&self, _delta: $val) {}
            fn set(&self, _val: $val) {}
        }
    );
}

