# Toy project to demonstrate fun stuff in rust

## gRPC

- `shared/proto` for definition
- `shared/src/build.rs` for generation
- `shared/src/toolbox/liveness.rs` for implementation
- `liveness/src/main.rs` for useage

## goose.rs load testing tool

- `liveness/src/main.rs` for a super simple endpoint
- `liveness-perftest/src/main.rs`for a simple test with validation of the endpoint
- start_liveness.sh to start the application
    then
- start_perf.sh to run a test scenario
- open `report.html` to open the report after a completed run, if it doesent auto open

## OpenTelemetry

- `shared/src/toolbox/telemetry.rs` for configuration of tracing, metrics, logging and connection between `tracing`(not otel) and the opentelemetry backend.
- `liveness/src/main.rs` for bootrapping

## configuration

- `shared/toolbox/config.rs` for implamentation
- `liveness/src/config.rs` for struct definition
- `liveness/src/main.rs` for useage to struct

## github build pipeline and security scanning

`.github/workflows/rust-check.yml` for security scanning
`.github/workflows/rust-build.yml` for building
