const baseArray = [
    new Map([["a", 1], ["b", 2]]),
    new Map([["a", "3"], ["b", "4"]]),
    new Map([["a", 5]]),
];

function checkMapValueEquivalence({ map1, map2, test_ctx }) {
    if (map1.size !== map2.size) {
        throw new Error(
            `${test_ctx} different number of keys (${map1.size} !== ${map2.size})`,
        );
    }
    for (const [key, value] of map1) {
        if (!map2.has(key)) {
            throw new Error(`${test_ctx} key ${key} not found in map2 keys`);
        }
        if (map2.get(key) !== value) {
            throw new Error(
                `${test_ctx} different values for key ${key} (${value} !== ${
                    map2.get(key)
                })`,
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
        checkMapValueEquivalence({
            map1: root.value[i],
            map2: baseArray[i],
            test_ctx: "first getRoot after commit",
        });
    }
}

const newObject = new Map([["a", 6], ["c", 7]]);
{
    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (!root.value) {
        throw new Error("root.value should exist");
    }

    // add a new object to the array
    root.value.push(newObject);
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
    if (root.value.length !== baseArray.length + 1) {
        throw new Error(
            `root.value should have length ${baseArray.length + 1}`,
        );
    }
    for (let i = 0; i < baseArray.length; i++) {
        checkMapValueEquivalence({
            map1: root.value[i],
            map2: baseArray[i],
            test_ctx: "added single object { a: 6, c: 7 }",
        });
    }

    // remove the 2nd object
    root.value = root.value.filter((_, i) => i !== 1);
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
    for (let i = 0; i < baseArray.length; i++) {
        checkMapValueEquivalence({
            map1: root.value[i],
            map2: baseArray[i],
            test_ctx: "removed object at index 1",
        });
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

    // clear the array
    root.value = [];
    commit();
}

{
    // check if the array is cleared (expect truthy)
    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (root.value.length !== 0) {
        throw new Error("root.value should have length 0");
    }
    if (root.value.length !== 0) {
        throw new Error("root.value should have length 0");
    }
}
