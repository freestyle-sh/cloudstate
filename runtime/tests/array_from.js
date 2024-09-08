{
    const baseArrayLike = { 0: "a", 1: "b", 2: "c", length: 3 };
    const object = {
        objValue: baseArrayLike,
    };

    setRoot("test-root", object);
    commit();
}

// END_FILE

{
    const expectedArrayLike = { 0: "a", 1: "b", 2: "c", length: 3 };
    const mapFn = (x) => x.toUpperCase();
    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (!root.objValue) {
        throw new Error("root.value should exist");
    }
    if (root.objValue.length !== expectedArrayLike.length) {
        throw new Error("root.value should have length 3");
    }

    const expectedArray = Array.from(expectedArrayLike, mapFn);
    const array = Array.from(root.objValue, mapFn);
    if (array.length !== expectedArray.length) {
        throw new Error("array should have length 3");
    }
    for (let i = 0; i < array.length; i++) {
        if (array[i] !== expectedArray[i]) {
            throw new Error(
                `value mismatch at index ${i}: ${array[i]} !== ${
                    expectedArray[i]
                }`,
            );
        }
    }

    root.arrValue = array;
    commit();
}

// END_FILE

{
    const expectedArray = ["A", "B", "C"];
    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (!root.arrValue) {
        throw new Error("root.arrValue should exist");
    }
    if (root.arrValue.length !== expectedArray.length) {
        throw new Error(
            "root.arrValue should have length ${expectedArray.length}",
        );
    }
    for (let i = 0; i < root.arrValue.length; i++) {
        if (root.arrValue[i] !== expectedArray[i]) {
            throw new Error(
                `value mismatch at index ${i}: ${root.arrValue[i]} !== ${
                    expectedArray[i]
                }`,
            );
        }
    }
}
