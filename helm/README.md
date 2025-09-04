# Eir Helm Chart

This Helm chart deploys the Eir metrics collector, a Rust-based service for collecting and exporting metrics from configurable HTTP endpoints.

## Features

- Deploys Eir as a containerized service
- Configurable clients, endpoints, and contexts via values.yaml
- Prometheus metrics exporter
- Supports custom environment variables
- ServiceMonitor resource for Prometheus Operator integration

## Installation

```sh
helm repo add margays https://margays.github.io/charts
helm install eir margays/eir
```

## Configuration

All configuration is managed via [`values.yaml`](values.yaml). Key sections:

- **image**: Container image settings
- **service**: Service type and port
- **config.clients**: HTTP client definitions (headers, max_connections)
- **config.endpoint_groups**: Endpoint groups with metrics definitions
- **config.contexts**: Contexts mapping clients to endpoint groups
- **env**: Additional environment variables

Example:

```yaml
config:
  clients:
    main:
      headers:
        Accept: application/vnd.github+json
        Authorization: Bearer <token>
        X-GitHub-Api-Version: 2022-11-28
        User-Agent: Prometheus
      max_connections: 10
  endpoint_groups:
    github:
      url: https://api.github.com/rate_limit
      interval: 1m
      metrics:
        - name: github_rate_limit_core_remaining
          description: Remaining core requests
          type: gauge
          value: jmespath:response.json.resources.core.remaining
          labels:
            - name: static
              value: some_static_value
  contexts:
    main:
      client: main
      endpoint_groups:
        - github
```

## Prometheus Integration

A ServiceMonitor resource is included for Prometheus Operator. Metrics are exposed at `/metrics` on the configured service port.
