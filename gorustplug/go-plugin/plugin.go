package main

import "fmt"

// Hello is the exported function that the main library will call
func Hello() string {
	fmt.Println("Plugin: Hello function called!")
	return "Hello from Go plugin!"
}

func main() {
	// This main function is required for plugin build mode
}
