const baseArray = [1, 2, 3, 4, 5, -10];

{
    const object = {
        value: baseArray,
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

    // test reduceRight with order verification on mutated array
    const operationIdxOrder = [];
    const result = root.value.reduceRight((acc, value, index) => {
        operationIdxOrder.push(index);
        return acc + value;
    }, 0);

    // Verify the result
    const expectedResult = [1, 2, 3, 4, 5, -10, 6, -11].reduceRight(
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
