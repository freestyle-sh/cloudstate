# Testing for JS Developers

> If you find a bug in cloudstate, the fastest way you can get it fixed is by
> contributing a failing JavaScript test.

If you have not already,
[install the rust toolchain](https://www.rust-lang.org/learn/get-started).

If you are using vscode, get the
[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
extension.

Fork this repository and clone it localy. You'll need to submit a PR request
with your failing test.

Tests are written in the /runtime/tests directory. Each test consists of 2 or
more transactions separated by `// END_FILE`. We surround each tests in a
closure for clarity of the variable namespace. No memory is persisted between
transactions so you cannot reuse variables past a `// END_FILE`.

Here's an example test.

```js
{
    const root = {
        value: "test",
    };

    setRoot("test-root", object);
}

// END_FILE

{
    const root = getRoot("test-value");
    if (root.value !== "test") {
        throw new Error("root.value did not equal test");
    }
}
```

The first transaction creates some data, and then uses `setRoot` to save that
data in cloudstate. The second transaction uses `getRoot` to retreive that data
from cloudstate and then verify that the data is as expected. Sometimes more
transactions are needed to test data in cloudstate being resaved and/or updated
after it's initially been saved. The first time an object is saved and the
second time an object is saved, 2 different mechanisms are used, so it's
important that many tests verify expected behavior.

To contribute tests, add a new js file to the runtime/tests folder, then add it
to the runtime/src/test.rs file. Please insert this test in alphabetical order.
(Look at the file that appears above your file in the vscode sidebar, and add
your new test right below that one.)
