{
    const object = {
        value: [],
    };
    setRoot("test-root", object);
    commit();
}

// END_FILE

{
    const object = getRoot("test-root");
    if (object.value.length !== 0) {
        throw new Error(`Expected length to be 0, got ${object.value.length}`);
    }

    object.value.push("a");
    commit();
}

// END_FILE

{
    const expected = ["a"];

    const object = getRoot("test-root");
    if (object.value.length !== expected.length) {
        throw new Error(`Expected length to be 1, got ${object.value.length}`);
    }
    for (let i = 0; i < expected.length; i++) {
        if (object.value[i] !== expected[i]) {
            throw new Error(`Expected ${expected[i]}, got ${object.value[i]}`);
        }
    }

    object.value.push(...["b", "c", "d", "e"]);
    commit();
}

// END_FILE

{
    const expected = ["a", "b", "c", "d", "e"];

    const object = getRoot("test-root");
    if (object.value.length !== expected.length) {
        throw new Error(`Expected length to be 5, got ${object.value.length}`);
    }
    for (let i = 0; i < expected.length; i++) {
        if (object.value[i] !== expected[i]) {
            throw new Error(`Expected ${expected[i]}, got ${object.value[i]}`);
        }
    }

    object.value.push("f");
    commit();
}

// END_FILE

{
    const expected = ["a", "b", "c", "d", "e", "f"];

    const object = getRoot("test-root");
    if (object.value.length !== expected.length) {
        throw new Error(
            `Expected length to be ${expected.length}, got ${object.value.length}`,
        );
    }
    for (let i = 0; i < expected.length; i++) {
        if (object.value[i] !== expected[i]) {
            throw new Error(`Expected ${expected[i]}, got ${object.value[i]}`);
        }
    }

    commit();
}
