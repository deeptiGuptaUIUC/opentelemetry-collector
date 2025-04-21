# Dynbatchprocessor

This folder within this repository is meant to capture code and necessary steps required to create a plugin using the Go [plugin package](https://pkg.go.dev/plugin). 

## Folder Structure

This folder contains the following files:
1. `main.go`: This is the boilerplate code for the plugin you want to create. All the methods defined in this file are necessary to load a plugin successfully in the OTel-Collector.
2. `go.mod`, `go.sum`: These files address the dependencies required to ensure the plugin matches exactly with the dependencies of the baseline collector.
3. `dynbatchprocessor.so`: This is the shared object artifact (plugin) created as part of this project.

## Steps to Create a Plugin (.so Artifact)

1. Create a `main.go` file as described in this folder. This code illustrates adding the `batchprocessor` component as a plugin. Feel free to bring your component as required.
2. In the same folder, run the following command to create a Go module:

   ```bash
   go mod init dynbatchprocessor
   ```

   This command will create `go.mod` and `go.sum` files in your current folder.

3. Run the following command to create the `.so` file in the same folder:

   ```bash
   go mod tidy && go build . && go build -o dynbatchprocessor.so -buildmode=plugin .
   ```

   This command will create two artifacts: `dynbatchprocessor` and `dynbatchprocessor.so`.

## Steps to Load the Plugin in OTel-Collector

1. Run the following command in the `cmd/otelcorecol` folder:

   ```bash
   go mod tidy && go build . && OTEL_COLLECTOR_SHARED_LIBRARY=<add absolute path to .so file> go run . --config test.yaml
   ```

   Replace the above path with the path to your `.so` file.

2. If you encounter the error:

   ```bash
   "plugin was built with a different version of package XXX"
   ```

   follow the steps below to resolve it.

## Dependency Management

To ensure compatibility, it is crucial that the plugin's dependencies match those of the baseline collector. Use the following steps to resolve dependency issues:

1. Use the following command to compare the Go version of the plugin and the baseline collector:

   ```bash
   go mod edit -go $(go mod edit -json /home/deeguptwsl/repos/github/opentelemetry-collector/cmd/otelcorecol/go.mod | jq -r .Go)
   ```

   > **Note**: Ensure `jq` is installed. You can install it using your package manager (e.g., `sudo apt install jq` on Ubuntu).

Another way of managing dependencies is to run the following command to identify where the dependency is coming from:

   ```bash
   go mod why <package name>
   ```
1. Modify the plugin's `go.mod` file to add, remove, or change the version of the dependency. You may need to add `replace` statements in the `go.mod` file.

2. Rebuild the plugin using the steps described above.

3. Repeat this process until all dependency issues are resolved.
