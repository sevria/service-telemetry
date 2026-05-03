# service-telemetry

Plug-and-play observability for Sevria backend services using OpenTelemetry.

## Configuration

The following environment variables can be used to configure the service:

| Variable              | Description                                         | Default         | Example           |
| --------------------- | --------------------------------------------------- | --------------- | ----------------- |
| `SERVICE_ENVIRONMENT` | Specifies the deployment environment.               | `development`   | `production`      |
| `SERVICE_INSTANCE_ID` | Unique identifier for the running service instance. | System hostname | `server.internal` |
