const baseArray = ["a", "b", "c", "d", "e"];

{
    const object = {
        value: baseArray,
    };
    setRoot("test-root", object);
    commit();
}

{
    const object = getRoot("test-root");
    if (object.value.length !== 5) {
        throw new Error(`Expected length to be 5, got ${object.value.length}`);
    }

    const shifted = object.value.shift();
    if (shifted !== "a") {
        throw new Error(`Expected "a", got ${shifted}`);
    }
    commit();
}

{
    const object = getRoot("test-root");
    if (object.value.length !== 4) {
        throw new Error(`Expected length to be 4, got ${object.value.length}`);
    }
    for (const [i, expected] of ["b", "c", "d", "e"].entries()) {
        if (object.value[i] !== expected) {
            throw new Error(
                `Expected ${expected} at index ${i}, got ${object.value[i]}`,
            );
        }
    }

    // clear the array
    object.value = [];
    commit();
}

{
    const object = getRoot("test-root");
    if (object.value.length !== 0) {
        throw new Error(`Expected length to be 0, got ${object.value.length}`);
    }

    // shift from an empty array
    const shifted = object.value.shift();
    if (shifted !== undefined) {
        throw new Error(`Expected undefined, got ${shifted}`);
    }
    commit();
}
