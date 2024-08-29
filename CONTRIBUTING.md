# Contributing to Cloudstate

Thank you for contributing to Cloudstate! We welcome contributions from everyone on the internet, and are grateful for even the smallest of fixes!

## Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/freestyle-sh/cloudstate
   ```

2. Go to the project directory:

   ```bash
   cd cloudstate
   ```

## Organization

| Directory | Description                                                                                   |
| --------- | --------------------------------------------------------------------------------------------- |
| `runtime` | The Cloudstate runtime, which is responsible for running the JavaScript and storing the data. |
| `server`  | The Cloudstate server, which is responsible for making the runtime accessible to consumers.   |
| `cli`     | The Cloudstate CLI, which is responsible for managing the Cloudstate server.                  |

## Getting Started

The place we most want help is testing the runtime. `runtime/tests` contains our current test suite. If you find any edge cases, or anything we're not testing that we should be, please follow the [writing tests](runtime/TESTING.md) to add a test.
