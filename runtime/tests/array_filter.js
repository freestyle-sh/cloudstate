const baseArray = [
    { id: 0, val: 3 },
    { id: 1, val: 0 },
    { id: 2, val: -10 },
    { id: 3, val: 0 },
    { id: 4, val: 3 },
    { id: 5, val: 16 },
];

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
    if (root.value.length !== baseArray.length) {
        throw new Error(`root.value should have length ${baseArray.length}`);
    }
    for (let i = 0; i < root.value.length; i++) {
        if (root.value.at(i).id !== baseArray[i].id) {
            throw new Error(
                `different values at index ${i} (${root.value.at(i).id} !== ${
                    baseArray[i].id
                })`,
            );
        }
        if (root.value.at(i).val !== baseArray[i].val) {
            throw new Error(
                `different values at index ${i} (${root.value.at(i).val} !== ${
                    baseArray[i].val
                })`,
            );
        }
    }

    // test filter
    const result = root.value.filter((item) => item.val > 0);

    // Verify the result
    const expectedResult = baseArray.filter(
        (item) => item.val > 0,
    );
    if (
        JSON.stringify(result) !== JSON.stringify(expectedResult)
    ) {
        throw new Error(
            `Result mismatch: ${JSON.stringify(result)} !== ${
                JSON.stringify(expectedResult)
            }`,
        );
    }

    // Verify the order
    const expectedIdxOrder = [0, 4, 5];
    const operationIdxOrder = result.map((item) => item.id);
    if (expectedIdxOrder.length !== operationIdxOrder.length) {
        throw new Error(
            `Order mismatch: ${JSON.stringify(operationIdxOrder)} !== ${
                JSON.stringify(expectedIdxOrder)
            }`,
        );
    }
    for (let i = 0; i < expectedIdxOrder.length; i++) {
        if (operationIdxOrder[i] !== expectedIdxOrder[i]) {
            throw new Error(
                `Order mismatch: ${JSON.stringify(operationIdxOrder)} !== ${
                    JSON.stringify(expectedIdxOrder)
                }`,
            );
        }
    }

    // test data persists after filter
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
    if (root.value.length !== baseArray.length) {
        throw new Error(`root.value should have length ${baseArray.length}`);
    }
    for (let i = 0; i < root.value.length; i++) {
        if (root.value.at(i).id !== baseArray[i].id) {
            throw new Error(
                `different values at index ${i} (${root.value.at(i).id} !== ${
                    baseArray[i].id
                })`,
            );
        }
        if (root.value.at(i).val !== baseArray[i].val) {
            throw new Error(
                `different values at index ${i} (${root.value.at(i).val} !== ${
                    baseArray[i].val
                })`,
            );
        }
    }
}
