# Cloudstate Development CLI

The cloudstate development CLI is a tool that helps you develop Cloudstate services locally. It provides a way to run a local cloudstate instance with either in memory or a local stateful store.

## Development

Start the local cloudstate instance with the following command:

```shell
cargo run -- serve test-counter.js --memory-only
```

Call the service with the following command:

```shell
curl -X POST -H "Content-Type: application/json" --data '{"params": []}' 0.0.0.0:3000/cloudstate/instances/counter/increment
```
