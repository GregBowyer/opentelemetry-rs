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

use std::borrow::Cow;
use std::collections::HashMap;

use crate::trace::span_context::SpanContext;
use crate::trace::span_id::SpanId;
use crate::trace::trace_id::TraceId;
use crate::trace::span::Span;
use crate::trace::attribute_value::AttributeValue;

pub enum ParentContext<'a> {
    /// The span has a remote parent
    RemoteParent(SpanContext<'a>),
    /// The span has a local parent
    Parent(SpanContext<'a>),
    /// The span is a root span, and is parentless
    RootSpan,
}

/// Sampling decision returned by `Sampler::should_sample`
pub trait Decision {
    /// Return sampling decision whether span should be sampled or not.
    fn is_sampled(&self) -> bool;

    /// Return tags which will be attached to the span.
    fn attributes(&self) -> HashMap<&str, &AttributeValue>;
}

/// Sampler is used to make decisions on {@link Span} sampling.
pub trait Sampler {
    type Decision: Decision;

    /// Called during `Span` creation to make a sampling decision.
    ///
    /// # Params
    /// * parentContext the parent span's `SpanContext`.
    /// * traceId the `TraceId` for the new `Span`.
    ///   This will be identical to that in the parentContext, unless this is a root span.
    /// * spanId the `SpanId` for the new `Span.
    /// * name the name of the new `Span`.
    /// * parentLinks the parentLinks associated with the new `Span.
    fn should_sample<'a, N, S>(&self, parent_ctx: ParentContext, trace_id: TraceId, span_id: SpanId,
                               name: N, parent_links: Vec<S>) -> Self::Decision
        where N: Into<Cow<'a, str>>,
              S: Span;

    /// Returns the description of this `Sampler`.
    ///
    /// This may be displayed on debug pages or in the logs.
    ///
    /// Example: `ProbabilitySampler{0.000100}`
    fn description(&self) -> &str;

}
