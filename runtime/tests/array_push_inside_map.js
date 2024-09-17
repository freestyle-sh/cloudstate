{
    setRoot("test-root", {
        value: new Map([[
            "a",
            {
                arr: [],
            },
        ]]),
    });
}

// END_FILE

{
    const object = getRoot("test-root");

    object.value.get("a").arr.push({
        a: 1,
    });
}

// END_FILE

{
    const object = getRoot("test-root");

    if (object.value.get("a").arr.length !== 1) {
        throw new Error(
            `Expected object.value.get("a").arr.length to be 1. Got ${
                object.value.get("a").arr.length
            }`,
        );
    }

    if (object.value.get("a").arr[0].a !== 1) {
        throw new Error(
            `Expected object.value.get("a").arr[0].a to be 1. Got ${
                object.value.get("a").arr[0].a
            }`,
        );
    }
}
