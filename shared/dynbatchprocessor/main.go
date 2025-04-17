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
		// Set default values for the configuration here.
		// For example:
		// 	Timeout: 5 * time.Second,
		// 	MaxSize: 100,
		// 	Timeout: 10 * time.Second,
		// 	BatchTimeout: 5 * time.Second,
		// 	BatchSendSize: 100,
		// 	BatchSendTimeout: 5 * time.Second,
		// 	BatchSendMaxSize: 1000,
		// 	BatchSendTimeout: 10 * time.Second
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
