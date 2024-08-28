const baseArray = [
    { a: 1, b: 2 },
    { a: 3, b: 4 },
    { a: 5 },
    { b: 6 },
    { c: 7 },
];

function checkObjectValueEquivalences({ obj1, obj2, test_ctx }) {
    const keys1 = Object.keys(obj1);
    const keys2 = Object.keys(obj2);
    if (keys1.length !== keys2.length) {
        throw new Error(
            `${test_ctx} different number of keys (${keys1.length} !== ${keys2.length})`,
        );
    }
    for (let i = 0; i < keys1.length; i++) {
        const keyToCheck = keys1[i];
        if (!keys2.includes(keyToCheck)) {
            throw new Error(
                `${test_ctx} key ${keyToCheck} not found in obj2 keys`,
            );
        }
        if (obj1[keyToCheck] !== obj2[keyToCheck]) {
            throw new Error(
                `${test_ctx} different values for key ${keyToCheck} (${
                    obj1[keyToCheck]
                } !== ${obj2[keyToCheck]})`,
            );
        }
    }
}

{
    const object = {
        value: [...baseArray],
    };

    setRoot("test-root", object);
    commit();
}

const newObject = { a: 8, c: 9 };
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
        checkObjectValueEquivalences({
            obj1: root.value.at(i),
            obj2: baseArray[i],
            test_ctx: "first getRoot after commit",
        });
    }

    // add a new object to the array
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
    checkObjectValueEquivalences({
        obj1: root.value.at(baseArray.length),
        obj2: newObject,
        test_ctx: "added single object { a: 8, c: 9 }",
    });

    // remove the 2nd object
    root.value = root.value.filter((_, index) => index !== 1);
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
    checkObjectValueEquivalences({
        obj1: root.value.at(1),
        obj2: baseArray.at(2),
        test_ctx: "removed 2nd object",
    });

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
    checkObjectValueEquivalences({
        obj1: root.value.at(0),
        obj2: baseArray.at(3),
        test_ctx: "removed objects with property 'a' (index 0)",
    });
    checkObjectValueEquivalences({
        obj1: root.value.at(1),
        obj2: baseArray.at(4),
        test_ctx: "removed objects with property 'a' (index 1)",
    });

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
    if (root.value.length !== 2) {
        throw new Error("root.value should have length 2");
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
