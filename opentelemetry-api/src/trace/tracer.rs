use std::borrow::Cow;
use crate::trace::span::Span;
use crate::context::Scope;

/// Tracer is a simple, interface for `Span` creation and in-process context interaction.
///
/// Users may choose to use manual or automatic Context propagation.
/// Because of that this class offers APIs to facilitate both usages.
///
/// <p>The automatic context propagation is done using {@link io.grpc.Context} which is a gRPC
/// independent implementation for in-process Context propagation mechanism which can carry
/// scoped-values across API boundaries and between threads. Users of the library must propagate the
/// {@link io.grpc.Context} between different threads.
///
/// ## Example usage with automatic context propagation:
///
/// <pre>{@code
/// class MyClass {
///   private static final Tracer tracer = OpenTelemetry.getTracer();
///   void doWork() {
///     Span span = tracer.spanBuilder("MyClass.DoWork").startSpan();
///     try(Scope ss = tracer.withSpan(span)) {
///       tracer.getCurrentSpan().addEvent("Starting the work.");
///       doWorkInternal();
///       tracer.getCurrentSpan().addEvent("Finished working.");
///     } finally {
///       span.end();
///     }
///   }
/// }
/// }</pre>
///
/// <p>Example usage with manual context propagation:
///
/// <pre>{@code
/// class MyClass {
///   private static final Tracer tracer = OpenTelemetry.getTracer();
///   void doWork(Span parent) {
///     Span childSpan = tracer.spanBuilder("MyChildSpan")
///         setParent(parent).startSpan();
///     childSpan.addEvent("Starting the work.");
///     try {
///       doSomeWork(childSpan); // Manually propagate the new span down the stack.
///     } finally {
///       // To make sure we end the span even in case of an exception.
///       childSpan.end();  // Manually end the span.
///     }
///   }
/// }
/// }</pre>
pub trait Tracer {
    type Span: Span;

    /// Gets the current Span from the current Context.
    ///
    /// To install a {@link Span} to the current Context use {@link #withSpan(Span)}.
    ///
    /// startSpan methods do NOT modify the current Context {@code Span}.
    ///
    /// @return a default {@code Span} that does nothing and has an invalid {@link SpanContext} if no
    ///    {@code Span} is associated with the current Context, otherwise the current {@code Span}
    ///    from the Context.
    fn current_span(&self) -> &Self::Span;

    /// Enters the scope of code where the given {@link Span} is in the current Context, and returns an
    /// object that represents that scope. The scope is exited when the returned object is closed.
    ///
    /// <p>Supports try-with-resource idiom.
    ///
    /// <p>Can be called with {@link DefaultSpan} to enter a scope of code where tracing is stopped.
    ///
    /// <p>Example of usage:
    ///
    /// <pre>{@code
    /// private static Tracer tracer = OpenTelemetry.getTracer();
    /// void doWork() {
    ///   // Create a Span as a child of the current Span.
    ///   Span span = tracer.spanBuilder("my span").startSpan();
    ///   try (Scope ws = tracer.withSpan(span)) {
    ///     tracer.getCurrentSpan().addEvent("my event");
    ///     doSomeOtherWork();  // Here "span" is the current Span.
    ///   }
    ///   span.end();
    /// }
    /// }</pre>
    ///
    /// <p>Prior to Java SE 7, you can use a finally block to ensure that a resource is closed
    /// regardless of whether the try statement completes normally or abruptly.
    ///
    /// <p>Example of usage prior to Java SE7:
    ///
    /// <pre>{@code
    /// private static Tracer tracer = OpenTelemetry.getTracer();
    /// void doWork() {
    ///   // Create a Span as a child of the current Span.
    ///   Span span = tracer.spanBuilder("my span").startSpan();
    ///   Scope ws = tracer.withSpan(span);
    ///   try {
    ///     tracer.getCurrentSpan().addEvent("my event");
    ///     doSomeOtherWork();  // Here "span" is the current Span.
    ///   } finally {
    ///     ws.close();
    ///   }
    ///   span.end();
    /// }
    /// }</pre>
    ///
    /// @param span The {@link Span} to be set to the current Context.
    /// @return an object that defines a scope where the given {@link Span} will be set to the current
    ///     Context.
    /// @throws NullPointerException if {@code span} is {@code null}.
    fn with_span<S: Scope>(&self, span: &Self::Span) -> S;

    /*
    /// Returns a {@link Span.Builder} to create and start a new {@link Span}.
    ///
    /// <p>See {@link Span.Builder} for usage examples.
    ///
    /// @param spanName The name of the returned Span.
    /// @return a {@code Span.Builder} to create and start a new {@code Span}.
    /// @throws NullPointerException if {@code spanName} is {@code null}.
    /// @since 0.1.0
    ///
    fn span_builder<'a, N: Into<Cow<'a, str>>>(&self, name: N) -> SpanBuilder<'a>;

    /// Records a `SpanData`.
    ///
    /// This API allows to send a pre-populated span object to the exporter.
    /// Sampling and recording decisions as well as other collection optimizations is a
    /// responsibility of a caller. Note, the `SpanContext` object on the span population with
    /// the values that will allow correlation of telemetry is also a caller responsibility.
    fn record_span_data(&self, span: SpanData);

    /// Returns the {@link BinaryFormat} for this tracer implementation.
    ///
    /// <p>If no tracer implementation is provided, this defaults to the W3C Trace Context binary
    /// format. For more details see <a href="https://w3c.github.io/trace-context-binary/">W3C Trace
    /// Context binary protocol</a>.
    ///
    /// <p>Example of usage on the client:
    ///
    /// <pre>{@code
    /// private static final Tracer tracer = OpenTelemetry.getTracer();
    /// private static final BinaryFormat binaryFormat = tracer.getBinaryFormat();
    /// void onSendRequest() {
    ///   Span span = tracer.spanBuilder("MyRequest").setSpanKind(Span.Kind.CLIENT).startSpan();
    ///   try (Scope ss = tracer.withSpan(span)) {
    ///     byte[] binaryValue = binaryFormat.toByteArray(tracer.getCurrentContext().context());
    ///     // Send the request including the binaryValue and wait for the response.
    ///   } finally {
    ///     span.end();
    ///   }
    /// }
    /// }</pre>
    ///
    /// <p>Example of usage on the server:
    ///
    /// <pre>{@code
    /// private static final Tracer tracer = OpenTelemetry.getTracer();
    /// private static final BinaryFormat binaryFormat = tracer.getBinaryFormat();
    /// void onRequestReceived() {
    ///   // Get the binaryValue from the request.
    ///   SpanContext spanContext = SpanContext.INVALID;
    ///   if (binaryValue != null) {
    ///     spanContext = binaryFormat.fromByteArray(binaryValue);
    ///   }
    ///   Span span = tracer.spanBuilder("MyRequest")
    ///       .setParent(spanContext)
    ///       .setSpanKind(Span.Kind.SERVER).startSpan();
    ///   try (Scope ss = tracer.withSpan(span)) {
    ///     // Handle request and send response back.
    ///   } finally {
    ///     span.end();
    ///   }
    /// }
    /// }</pre>
    ///
    /// @return the {@code BinaryFormat} for this implementation.
    /// @since 0.1.0
    ///
    fn get_binary_format(&self) -> BinaryFormat<SpanContext>;

    /// Returns the {@link HttpTextFormat} for this tracer implementation.
    ///
    /// <p>If no tracer implementation is provided, this defaults to the W3C Trace Context HTTP text
    /// format ({@link io.opentelemetry.context.propagation.TraceContextFormat}). For more details see
    /// <a href="https://w3c.github.io/trace-context/">W3C Trace Context</a>.
    ///
    /// <p>Example of usage on the client:
    ///
    /// <pre>{@code
    /// private static final Tracer tracer = OpenTelemetry.getTracer();
    /// private static final HttpTextFormat textFormat = tracer.get_http_text_format();
    /// private static final HttpTextFormat.Setter setter =
    ///         new HttpTextFormat.Setter<HttpURLConnection>() {
    ///   public void put(HttpURLConnection carrier, String key, String value) {
    ///     carrier.setRequestProperty(field, value);
    ///   }
    /// }
    ///
    /// void makeHttpRequest() {
    ///   Span span = tracer.spanBuilder("MyRequest").setSpanKind(Span.Kind.CLIENT).startSpan();
    ///   try (Scope s = tracer.withSpan(span)) {
    ///     HttpURLConnection connection =
    ///         (HttpURLConnection) new URL("http://myserver").openConnection();
    ///     textFormat.inject(span.getContext(), connection, httpURLConnectionSetter);
    ///     // Send the request, wait for response and maybe set the status if not ok.
    ///   }
    ///   span.end();  // Can set a status.
    /// }
    /// }</pre>
    ///
    /// <p>Example of usage on the server:
    ///
    /// <pre>{@code
    /// private static final Tracer tracer = OpenTelemetry.getTracer();
    /// private static final HttpTextFormat textFormat = tracer.get_http_text_format();
    /// private static final HttpTextFormat.Getter<HttpRequest> getter = ...;
    ///
    /// void onRequestReceived(HttpRequest request) {
    ///   SpanContext spanContext = textFormat.extract(request, getter);
    ///   Span span = tracer.spanBuilder("MyRequest")
    ///       .setParent(spanContext)
    ///       .setSpanKind(Span.Kind.SERVER).startSpan();
    ///   try (Scope s = tracer.withSpan(span)) {
    ///     // Handle request and send response back.
    ///   }
    ///   span.end()
    /// }
    /// }</pre>
    ///
    /// @return the {@code HttpTextFormat} for this implementation.
    /// @since 0.1.0
    ///
    fn get_http_text_format(&self) -> HttpTextFormat<SpanContext>;
    */
}