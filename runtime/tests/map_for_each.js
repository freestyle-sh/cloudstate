const map = new Map([
    ["foo", 1],
    ["bar", 2],
    ["baz", 3],
]);

{
    const object = {
        value: map,
    };

    setRoot("test-root", object);
    commit();
}

{
    const object = getRoot("test-root");
    let count = 0;
    object.value.forEach((value, key) => {
        if (map.get(key) !== value) {
            throw new Error(
                `Expected ${JSON.stringify(map.get(key))}, got ${
                    JSON.stringify(value)
                }`,
            );
        }
        if (object.value.get(key) !== value) {
            throw new Error(
                `Expected ${JSON.stringify(object.value.get(key))}, got ${
                    JSON.stringify(value)
                }`,
            );
        }
        count++;
    });
    if (count !== 3) {
        throw new Error(`Expected 3 iterations, got ${count}`);
    }
}
