# Collector Componentization

## Overview

The OpenTelemetry Collector Contrib repository contains around 300 components for customers to choose from when building their data collection pipelines. Componentization provides a framework for customers to easily pick components without dealing with the complexity of building the entire monolithic repository. This is particularly important for addressing vulnerabilities in data pipelines, allowing customers to update specific components without rebuilding the entire collector.

Additionally, many teams at Microsoft use their own versions of the OpenTelemetry Collector distro, facing challenges such as bug fixes, security patches, safe builds, and signing. Componentization aims to address these challenges by enabling a pluggable model for the collector.

## Goals

1. **Seamless Integration:** Allow customers to integrate components from the OpenTelemetry Collector Contrib repository as plugins into their existing collector instances.
2. **Security:** Ensure secure interaction between plugins and the baseline collector.
3. **Performance:** Focus on high throughput and low latency ingestion.
4. **Contribution to the OpenTelemetry Community:** Contribute the solution upstream to the OpenTelemetry community.

## Out Of Scope

Details on how to transition from the current state to the proposed solution are considered out of scope and will be discussed in individual PRs.

## Proposed Solution

### Pluggable Model

The collector will support a pluggable model, enabling runtime extension by loading external components (plugins) without recompiling the main collector binary. This model provides:

1. **Flexibility:** New features or components can be added without rebuilding or redeploying the entire collector.
2. **Isolation:** Teams can develop, test, and release external components independently.
3. **Customization:** Customers can tailor their collector instance by selecting only the components they need.

### Plugin Formats

#### In-Process Plugin

Plugins interact with the baseline collector dynamically within the same process address space. The collector scans for plugins based on configuration metadata at startup, loads each plugin, and registers the provided components.

#### Out-of-Process Plugin

Plugins run in separate child processes, with the baseline collector managing communication via gRPC. This approach provides strong isolation and resilience, ensuring the stability of the main collector process.

### Risks and Mitigations

1. **Dependency Mismatch:** Provide customers with details of the Go build environment for each collector release to ensure compatibility.
2. **Performance Overhead:** While IPC introduces overhead, the benefits of process isolation and resilience outweigh the drawbacks.

## Implementation Details

### Go Plugin Creation

The design utilizes the existing [Go plugin module](https://pkg.go.dev/plugin). Below is a sample Go plugin code for a batch processor:

```go
package main

import (
    "C" // Required for building Go plugins, enables Cgo
    "go.opentelemetry.io/collector/component"
    "go.opentelemetry.io/collector/processor"
    "go.opentelemetry.io/collector/processor/batchprocessor"
)

// NewFactory exports the factory for the batchprocessor.
func NewFactory() processor.Factory {
    return batchprocessor.NewFactory()
}

// CreateDefaultConfig exports a function to create the default configuration for the processor.
func CreateDefaultConfig() component.Config {
    return batchprocessor.NewFactory().CreateDefaultConfig()
}

// main is required by the Go plugin build mode. It does not run when the plugin is loaded.
func main() {
}
```

To build the code above, use the following command:

```bash
go build -o <name_of_your_plugin>.so -buildmode=plugin main.go
```

### Configuration Metadata

Each plugin must be accompanied by a configuration metadata file. This file provides the collector with information on how to load and use the plugin.

```yaml
path: /path/to/your_plugin.so # Absolute or relative path to the .so plugin file
name: <unique_component_name>   # Name used in the collector's configuration (e.g., mybatchprocessor)
type: <receiver|processor|exporter> # Type of the component (e.g., processor)
data_type: <traces|metrics|logs>    # Telemetry data type handled (e.g., traces)
version: "1.0.0"
description: "A custom batch processor plugin."
```

### Out-of-Process Architecture

In this approach, a new child process is created to run the plugin. The baseline collector is responsible for launching, managing, and communicating with the customer's plugin (hosted in a child process), based on the configuration metadata of the plugin. Communication occurs via gRPC using the OTLP protocol.

**Key characteristics:**

1. **Process Isolation:** The out-of-process component runs in a separate process, providing strong isolation. If the component crashes or misbehaves, it does not affect the stability of the main collector process.
2. **Resilience:** The main collector monitors the health of the child process. If the plugin fails to start, crashes, or the communication channel is broken, the collector can log the error and continue running.

## References

* [Fluentbit - Golang Output Plugins](https://docs.fluentbit.io/manual/development/golang-output-plugins)
* [Hashicorp solution](https://pkg.go.dev/github.com/hashicorp/go-plugin)
* [Previous discussion on plugin componentization](https://github.com/open-telemetry/opentelemetry-collector/issues/1005)
