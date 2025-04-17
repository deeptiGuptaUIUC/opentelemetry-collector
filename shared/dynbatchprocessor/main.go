package main

import (
	"C"

	"go.opentelemetry.io/collector/component"
	"go.opentelemetry.io/collector/processor"
	"go.opentelemetry.io/collector/processor/batchprocessor"
)

// Config represents the configuration for the processor.
type Config struct {
	//component.ProcessorConfig
}

// CreateDefaultConfig creates the default configuration for the processor.
func CreateDefaultConfig() component.Config {
	return &Config{
		// ProcessorConfig: component.ProcessorConfig{
		// 	TypeVal: "Deepti",
		// 	NameVal: "Deepti",
		// },
	}
}

func NewFactory() processor.Factory {
	return batchprocessor.NewFactory()
}

func main() {
	// In the real world, be polite about it, or,
	// In the real world, this is the entry point for a Collector that uses RPC.
	panic("NOT USED")
}
