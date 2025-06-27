package main

import (
	"C"
	"fmt"
	"plugin"
	"runtime"
)

//export LoadAndCallPlugin
func LoadAndCallPlugin(pluginPath *C.char) C.int {
	fmt.Println("=== Go Main Library: Starting plugin load ===")
	fmt.Printf("Go runtime info: NumGoroutine=%d, GOMAXPROCS=%d\n",
		runtime.NumGoroutine(), runtime.GOMAXPROCS(0))

	goPluginPath := C.GoString(pluginPath)
	fmt.Printf("Attempting to load plugin: %s\n", goPluginPath)

	// Load the plugin
	p, err := plugin.Open(goPluginPath)
	if err != nil {
		fmt.Printf("Failed to load plugin: %v\n", err)
		return 1
	}

	fmt.Println("Plugin loaded successfully!")

	// Look for a Hello function in the plugin
	sym, err := p.Lookup("Hello")
	if err != nil {
		fmt.Printf("Failed to find Hello function: %v\n", err)
		return 1
	}

	// Call the Hello function
	helloFunc, ok := sym.(func() string)
	if !ok {
		fmt.Println("Hello function has wrong signature")
		return 1
	}

	result := helloFunc()
	fmt.Printf("Plugin Hello() returned: %s\n", result)

	fmt.Println("=== Go Main Library: Plugin interaction successful ===")
	return 0
}

//export GetRuntimeInfo
func GetRuntimeInfo() *C.char {
	info := fmt.Sprintf("Go runtime: %d goroutines, GOMAXPROCS=%d",
		runtime.NumGoroutine(), runtime.GOMAXPROCS(0))
	return C.CString(info)
}

func main() {
	// This main function is required for c-shared build mode
	fmt.Println("Go main library initialized")
}
