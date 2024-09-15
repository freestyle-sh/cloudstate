{
    const base = [1, 4, 9, 16];
    const object = {
        value: base,
    };
    setRoot("test-root", object);
}

// END_FILE

{
    const mapFn = (element) => element * 2;
    const expected = [1, 4, 9, 16].map(mapFn);

    const object = getRoot("test-root");
    const newArray = object.value.map(mapFn);
    if (newArray.length !== expected.length) {
        throw new Error(
            `Expected length to be ${expected.length}, got ${newArray.length}`,
        );
    }
    for (let i = 0; i < newArray.length; i++) {
        if (newArray[i] !== expected[i]) {
            throw new Error(
                `Expected ${JSON.stringify(expected[i])}, got ${
                    JSON.stringify(
                        newArray[i],
                    )
                }`,
            );
        }
    }
}
