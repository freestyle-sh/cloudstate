const baseArray = [
    { a: "1", b: "2" },
    { a: "3", b: "4" },
    { a: "5" },
    { b: "6" },
    { c: "7" },
];

{
    const object = {
        value: [...baseArray],
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

    // check at for each item
    for (let i = 0; i < root.value.length; i++) {
        if (root.value.at(i) !== baseArray[i]) {
            throw new Error(
                `value mismatch at index ${i}: ${root.value.at(i)} !== ${
                    baseArray[i]
                }`,
            );
        }
    }

    // add a new object to the array
    const newObject = { a: "8", c: "9" };
    root.value.push(newObject);
    commit();
}

{
    // check if the new object is added
    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (!root.value) {
        throw new Error("root.value should exist");
    }
    if (root.value.length !== baseArray.length + 1) {
        throw new Error(
            `root.value should have length ${baseArray.length + 1}`,
        );
    }
    if (root.value.at(baseArray.length) !== newObject) {
        throw new Error(
            `value mismatch at index ${baseArray.length}: ${
                root.value.at(
                    baseArray.length,
                )
            } !== ${newObject}`,
        );
    }

    // remove the 2nd object
    root.value.splice(1, 1);
    commit();
}

{
    // check if the 2nd object is removed
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
    if (root.value.at(1) !== baseArray[2]) {
        throw new Error(
            `value mismatch at index 1: ${root.value.at(1)} !== ${
                baseArray[2]
            }`,
        );
    }

    // remove all objects with a property "a"
    root.value = root.value.filter((item) => !item.hasOwnProperty("a"));
    commit();
}

{
    // check if objects with property "a" are removed
    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (!root.value) {
        throw new Error("root.value should exist");
    }
    if (root.value.length !== 2) {
        throw new Error("root.value should have length 2");
    }
    if (root.value.at(0) !== baseArray[3]) {
        throw new Error(
            `value mismatch at index 0: ${root.value.at(0)} !== ${
                baseArray[3]
            }`,
        );
    }
    if (root.value.at(1) !== baseArray[4]) {
        throw new Error(
            `value mismatch at index 1: ${root.value.at(1)} !== ${
                baseArray[4]
            }`,
        );
    }

    // zero-mutation commit
    commit();
}

{
    // check if the array is cleared (expect falsy)
    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (!root.value) {
        throw new Error("root.value should exist");
    }
    root.value = [];
    commit();
}

{
    // check if the array is cleared (expect truthy)
    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (!root.value) {
        throw new Error("root.value should exist");
    }
    if (root.value.length !== 0) {
        throw new Error("root.value should have length 0");
    }
}
