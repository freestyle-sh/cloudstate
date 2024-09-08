const baseArray = [1, 2, 3, 4, 5, -10];

{
    const object = {
        value: baseArray,
    };

    setRoot("test-root", object);
    commit();
}

// END_FILE

{
    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (!root.value) {
        throw new Error("root.value should exist");
    }
    if (root.value.length !== 6) {
        throw new Error(`root.value should have length 6`);
    }
    for (const [i, expected] of [1, 2, 3, 4, 5, -10].entries()) {
        if (root.value[i] !== expected) {
            throw new Error(
                `expected ${expected} at index ${i}, got ${root.value[i]}`,
            );
        }
    }

    // test reduce with index order verification
    const processedIdxOrder = [];
    const result = root.value.reduce((acc, value, index) => {
        processedIdxOrder.push(index);
        return acc + value;
    }, 0);

    // Verify the result
    const expectedResult = [1, 2, 3, 4, 5, -10].reduce(
        (acc, value) => acc + value,
        0,
    );
    if (result !== expectedResult) {
        throw new Error(`Result mismatch: ${result} !== ${expectedResult}`);
    }

    // Verify the index order
    const expectedIdxOrder = [0, 1, 2, 3, 4, 5]; // LTR
    if (
        JSON.stringify(processedIdxOrder) !== JSON.stringify(expectedIdxOrder)
    ) {
        throw new Error(
            `Index order mismatch: ${JSON.stringify(processedIdxOrder)} !== ${
                JSON.stringify(expectedIdxOrder)
            }`,
        );
    }

    // mutate array
    root.value.push(6);
    root.value.push(-11);
    commit();
}

// END_FILE

{
    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (!root.value) {
        throw new Error("root.value should exist");
    }
    if (root.value.length !== 8) {
        throw new Error(
            `root.value should have length 8`,
        );
    }
    for (const [i, expected] of [1, 2, 3, 4, 5, -10, 6, -11].entries()) {
        if (root.value[i] !== expected) {
            throw new Error(
                `expected ${expected} at index ${i}, got ${root.value[i]}`,
            );
        }
    }

    // test reduce with index order verification on mutated array
    const processedIdxOrder = [];
    const result = root.value.reduce((acc, value, index) => {
        processedIdxOrder.push(index);
        return acc + value;
    }, 0);

    // Verify the result
    const expectedResult = [1, 2, 3, 4, 5, -10, 6, -11].reduce(
        (acc, value) => acc + value,
        0,
    );
    if (result !== expectedResult) {
        throw new Error(
            `Result mismatch after mutation: ${result} !== ${expectedResult}`,
        );
    }

    // Verify the index order
    const expectedIdxOrder = [0, 1, 2, 3, 4, 5, 6, 7]; // LTR
    if (
        JSON.stringify(processedIdxOrder) !== JSON.stringify(expectedIdxOrder)
    ) {
        throw new Error(
            `Index order mismatch after mutation: ${
                JSON.stringify(processedIdxOrder)
            } !== ${JSON.stringify(expectedIdxOrder)}`,
        );
    }
}
