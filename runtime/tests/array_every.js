const base = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

{
    const object = {
        value: [...base],
    };

    setRoot("test-root", object);
    commit();
}

{
    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (!root.value) {
        throw new Error("root.value should exist");
    }
    if (root.value.length !== base.length) {
        throw new Error(`root.value should have length ${base.length}`);
    }
    // check less than 11
    if (!root.value.every((v) => v < 11)) {
        throw new Error(`every value should be less than 11`);
    }
    // check greater than 0
    if (!root.value.every((v) => v > 0)) {
        throw new Error(`every value should be greater than 0`);
    }
    // check odd
    if (root.value.every((v) => v % 2 === 1)) {
        throw new Error(`every value is not odd`);
    }
    // check every value is one more than its index
    if (!root.value.every((v, i) => v === i + 1)) {
        throw new Error(`every value should be one more than its index`);
    }
    // check every value is equal to it in the array parameter
    if (!root.value.every((v, i, a) => v === a[i])) {
        throw new Error(
            `every value should be equal to it in the array parameter`,
        );
    }
}
