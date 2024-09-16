{
    setRoot("test-root", {
        value: [1],
    });
}

// END_FILE

{
    const object = getRoot("test-root");

    const value = object.value.pop();
    if (value !== 1) {
        throw new Error(`Expected first popped value to be 1, got ${value}`);
    }

    if (object.value[0] !== undefined) {
        throw new Error(
            `Array element was not removed from the array when popped in current transaction`,
        );
    }

    if (object.value.length !== 0) {
        throw new Error(
            `Array's length should be 0 after popping an element in current transaction`,
        );
    }
}

// END_FILE

{
    const object = getRoot("test-root");

    if (object.value[0] !== undefined) {
        throw new Error(
            `Array element should be undefined after popping an element in previous transaction`,
        );
    }

    if (object.value.length !== 0) {
        throw new Error(
            `Array's length should be 0 after popping an element in previous transaction`,
        );
    }
}
