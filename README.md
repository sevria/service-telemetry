# sevria-service-telemetry

[![Github Actions](https://github.com/sevria/service-telemetry/actions/workflows/publish.yaml/badge.svg)](https://github.com/sevria/service-telemetry/actions)
[![crates.io](https://img.shields.io/crates/v/sevria-service-telemetry.svg)](https://crates.io/crates/sevria-service-telemetry)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Plug-and-play observability for Sevria backend services using OpenTelemetry.

## Installation

Add the crate to your project using Cargo:

```bash
cargo add sevria-service-telemetry
```

## Configuration

The following environment variables can be used to configure the service:

| Variable              | Description                                         | Default         | Example           |
| --------------------- | --------------------------------------------------- | --------------- | ----------------- |
| `SERVICE_ENVIRONMENT` | Specifies the deployment environment.               | `development`   | `production`      |
| `SERVICE_INSTANCE_ID` | Unique identifier for the running service instance. | System hostname | `server.internal` |
