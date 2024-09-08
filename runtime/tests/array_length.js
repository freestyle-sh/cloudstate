{
    const base = [1, 2, 3, 4, 5];
    const object = {
        value1: [],
        value2: base,
    };

    setRoot("test-root", object);
    commit();
}

// END_FILE

{
    const object = getRoot("test-root");
    if (object.value1.length !== 0) {
        throw new Error(
            `Expected value1 length to be 0, got ${object.value1.length}`,
        );
    }
    if (object.value2.length !== 5) {
        throw new Error(
            `Expected value2 length to be 5, got ${object.value2.length}`,
        );
    }
}
