{
    const base = [1, 2, 3, 4, 5, -10];
    const object = {
        value: base,
    };
    setRoot("test-root", object);
    commit();
}

// END_FILE

{
    const expectedArr = [1, 2, 3, 4, 5, -10];

    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (!root.value) {
        throw new Error("root.value should exist");
    }
    if (root.value.length !== expectedArr.length) {
        throw new Error(
            `root.value should have length ${expectedArr.length}, got ${root.value.length}`,
        );
    }
    for (const [i, expected] of expectedArr.entries()) {
        if (root.value[i] !== expected) {
            throw new Error(
                `expected ${expected} at index ${i}, got ${root.value[i]}`,
            );
        }
    }

    // test reduceRight with order verification
    const operationIdxOrder = [];
    const result = root.value.reduceRight((acc, value, index) => {
        operationIdxOrder.push(index);
        return acc + value;
    }, 0);

    // Verify the result
    const expectedResult = [1, 2, 3, 4, 5, -10].reduceRight(
        (acc, value) => acc + value,
        0,
    );
    if (result !== expectedResult) {
        throw new Error(`Result mismatch: ${result} !== ${expectedResult}`);
    }

    // Verify the order
    const expectedIdxOrder = [5, 4, 3, 2, 1, 0]; // RTL
    if (
        JSON.stringify(operationIdxOrder) !== JSON.stringify(expectedIdxOrder)
    ) {
        throw new Error(
            `Order mismatch: ${JSON.stringify(operationIdxOrder)} !== ${
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
    const expectedArr = [1, 2, 3, 4, 5, -10, 6, -11];

    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (!root.value) {
        throw new Error("root.value should exist");
    }
    console.log(root.value.entries());
    if (root.value.length !== expectedArr.length) {
        throw new Error(
            `root.value should have length ${expectedArr.length}, got ${root.value.length}`,
        );
    }
    for (const [i, expected] of expectedArr.entries()) {
        if (root.value[i] !== expected) {
            throw new Error(
                `expected ${expected} at index ${i}, got ${root.value[i]}`,
            );
        }
    }

    // test reduceRight with order verification on mutated array
    const operationIdxOrder = [];
    const result = root.value.reduceRight((acc, value, index) => {
        operationIdxOrder.push(index);
        return acc + value;
    }, 0);

    // Verify the result
    const expectedResult = expectedArr.reduceRight(
        (acc, value) => acc + value,
        0,
    );
    if (result !== expectedResult) {
        throw new Error(
            `Result mismatch after mutation: ${result} !== ${expectedResult}`,
        );
    }

    // Verify the order
    const expectedIdxOrder = [7, 6, 5, 4, 3, 2, 1, 0]; // RTL
    if (
        JSON.stringify(operationIdxOrder) !== JSON.stringify(expectedIdxOrder)
    ) {
        throw new Error(
            `Order mismatch after mutation: ${
                JSON.stringify(operationIdxOrder)
            } !== ${JSON.stringify(expectedIdxOrder)}`,
        );
    }
}
