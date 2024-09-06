const baseArray = [
    new Map([["a", 1], ["b", 2]]),
    new Map([["a", "3"], ["b", "4"]]),
    new Map([["a", 5]]),
];

function checkMapValueEquivalence({ map1, map2, test_ctx }) {
    console.log("map1", map1);
    console.log("map2", map2);
    if (map1.size !== map2.size) {
        throw new Error(
            `${test_ctx} different number of keys (${map1.size} !== ${map2.size})`,
        );
    }
    for (const [key, value] of map1.entries()) {
        console.log("key of map1", key);
        console.log("value of map1", value);
        if (!map2.has(key)) {
            throw new Error(`${test_ctx} key ${key} not found in map2 keys`);
        }
        if (map2.get(key) !== value) {
            console.log("map2.get(key)", map2.get(key));
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
    if (root.value.length !== 3) {
        throw new Error(`root.value should have length 3`);
    }
    for (
        const [i, expected] of [
            new Map([["a", 1], ["b", 2]]),
            new Map([["a", "3"], ["b", "4"]]),
            new Map([["a", 5]]),
        ].entries()
    ) {
        checkMapValueEquivalence({
            map1: root.value[i],
            map2: expected,
            test_ctx: "first getRoot after commit",
        });
    }
}

const newMap = new Map([["a", 6], ["c", 7]]);
{
    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (!root.value) {
        throw new Error("root.value should exist");
    }

    // add a new map to the array
    root.value.push(newMap);
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
    if (root.value.length !== 4) {
        throw new Error(
            `root.value should have length 4`,
        );
    }
    for (
        const [i, expected] of [
            new Map([["a", 1], ["b", 2]]),
            new Map([["a", "3"], ["b", "4"]]),
            new Map([["a", 5]]),
            new Map([["a", 6], ["c", 7]]),
        ].entries()
    ) {
        checkMapValueEquivalence({
            map1: root.value[i],
            map2: expected,
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
    if (root.value.length !== 3) {
        throw new Error(`root.value should have length 3`);
    }
    for (
        const [i, expected] of [
            new Map([["a", 1], ["b", 2]]),
            new Map([["a", 5]]),
            new Map([["a", 6], ["c", 7]]),
        ].entries()
    ) {
        checkMapValueEquivalence({
            map1: root.value[i],
            map2: expected,
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
