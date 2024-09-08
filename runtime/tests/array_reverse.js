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

    object.value.reverse();
    commit();
}

// END_FILE

{
    const base = ["a", "b", "c", "d", "e"];

    const object = getRoot("test-root");
    if (object.value.length !== 0) {
        throw new Error(`Expected length to be 0, got ${object.value.length}`);
    }

    object.value.push(...base);
    object.value.reverse();
    commit();
}

// END_FILE

{
    const expected = ["e", "d", "c", "b", "a"];

    const object = getRoot("test-root");
    if (object.value.length !== expected.length) {
        throw new Error(
            `Expected length to be ${expected.length}, got ${object.value.length}`,
        );
    }
    for (let i = 0; i < expected.length; i++) {
        if (object.value[i] !== expected[i]) {
            throw new Error(
                `Expected array item ${base[i]} at index ${i}, got ${
                    object.value[i]
                }}`,
            );
        }
    }
}
