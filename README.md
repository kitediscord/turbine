# Kite Turbine

#### The core service of kite which does all the heavy lifting

## Crates

### server

The entrypoint which connects all the components together. It mainly provides a gRPC interface for interacting with the
internal components and connects to the Discord gateway to receive events.

### components/bot

Small abstractions on top of the APIs provided by [twilight](https://github.com/twilight-rs/twilight). Everything that
wants to interact with the Discord API will make use of this.

### components/engine

Instantiates plugins and handles the communication between the host and the guest code.

### components/scheduler

Retrieves and schedules plugins using the engine when a relevant event is received.

### components/store

Contains interfaces (aka. traits) and implementations for interacting with different databases.
