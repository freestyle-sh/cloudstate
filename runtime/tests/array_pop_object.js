{
    setRoot("test-root", {
        value: [{
            a: 1,
        }],
    });
}

// END_FILE

{
    const object = getRoot("test-root");

    const value = object.value.pop();

    if (JSON.stringify(value) !== JSON.stringify({ a: 1 })) {
        throw new Error(
            `Expected first popped value to be { a: 1 }, got ${
                JSON.stringify(value)
            }`,
        );
    }
}
