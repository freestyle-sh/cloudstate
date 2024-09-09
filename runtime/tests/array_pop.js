{
    const base = ["a", "b", "c", "d", "e"];
    const object = {
        value: base,
    };
    setRoot("test-root", object);
    commit();
}

// END_FILE

{
    const expected = ["a", "b", "c", "d", "e"];

    const object = getRoot("test-root");
    if (object.value.length !== expected.length) {
        throw new Error(
            `Expected length to be ${expected.length}, got ${object.value.length}`,
        );
    }

    const popped = object.value.pop();
    if (popped !== "e") {
        throw new Error(`Expected "e", got ${popped}`);
    }
    commit();
}

// END_FILE

{
    const expected = ["a", "b", "c", "d"];

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

    const popped = object.value.pop();
    if (popped !== "d") {
        throw new Error(`Expected "d", got ${popped}`);
    }
    commit();
}

// END_FILE

{
    const expected = ["a", "b", "c"];

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

    const popped = object.value.pop();
    if (popped !== "c") {
        throw new Error(`Expected "c", got ${popped}`);
    }
    commit();
}

// END_FILE

{
    const expected = ["a", "b"];

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

    const popped = object.value.pop();
    if (popped !== "b") {
        throw new Error(`Expected "b", got ${popped}`);
    }
    commit();
}

// END_FILE

{
    const expected = ["a"];

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

    const popped = object.value.pop();
    if (popped !== "a") {
        throw new Error(`Expected "a", got ${popped}`);
    }
    commit();
}

// END_FILE

{
    const object = getRoot("test-root");
    if (object.value.length !== 0) {
        throw new Error(`Expected length to be 0, got ${object.value.length}`);
    }

    const popped = object.value.pop();
    if (popped !== undefined) {
        throw new Error(`Expected undefined, got ${popped}`);
    }
}
