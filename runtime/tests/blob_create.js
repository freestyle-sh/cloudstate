{
    setRoot("test-root", {
        value: {
            a: new Blob(["hello"], {
                type: "text/plain",
            }),
        },
    });
}

// END_FILE

{
    const object = getRoot("test-root");

    if (!(object.value.a instanceof Blob)) {
        throw new Error(
            `Expected object.value.a to be a Blob. Got ${object.value.a}`,
        );
    }

    if (object.value.a.size !== 5) {
        throw new Error(
            `Expected object.value.a.size to be 5. Got ${object.value.a.size}`,
        );
    }

    if (object.value.a.type !== "text/plain") {
        throw new Error(
            `Expected object.value.a.type to be "text/plain". Got ${object.value.a.type}`,
        );
    }

    const text = await object.value.a.text();

    if (text !== "hello") {
        throw new Error(
            `Expected object.value.a.text() to return "hello". Got ${text}`,
        );
    }

    // changing object to force a commit
    object.value.b = "test";
}

// END_FILE

{
    const object = getRoot("test-root");

    // checking that blobs are saved correctly
    if (await object.value.a.text() !== "hello") {
        throw new Error(
            `Expected object.value.a to be "hello". Got "${object.value.a}"`,
        );
    }
}
