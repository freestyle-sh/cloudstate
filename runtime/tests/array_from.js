// Initialize data
const arrayLike = { 0: "a", 1: "b", 2: "c", length: 3 };
const mapFn = (x) => x.toUpperCase();

{
    const object = {
        value: Array.from(arrayLike, mapFn),
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

    // Check if Array.from() worked correctly
    if (root.value.length !== 3) {
        throw new Error(`Expected length 3, got ${root.value.length}`);
    }
    if (
        root.value[0] !== "A" || root.value[1] !== "B" || root.value[2] !== "C"
    ) {
        throw new Error(
            `Expected ['A', 'B', 'C'], got ${JSON.stringify(root.value)}`,
        );
    }

    // Test Array.from() with a string
    root.value = Array.from("hello");
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

    // Check if Array.from() worked correctly with a string
    if (root.value.length !== 5) {
        throw new Error(`Expected length 5, got ${root.value.length}`);
    }
    if (root.value.join("") !== "hello") {
        throw new Error(
            `Expected ['h','e','l','l','o'], got ${JSON.stringify(root.value)}`,
        );
    }

    // Test Array.from() with Set
    const set = new Set(["foo", "bar", "baz"]);
    root.value = Array.from(set);
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

    // Check if Array.from() worked correctly with Set
    if (root.value.length !== 3) {
        throw new Error(`Expected length 3, got ${root.value.length}`);
    }
    if (
        !root.value.includes("foo") || !root.value.includes("bar") ||
        !root.value.includes("baz")
    ) {
        throw new Error(
            `Expected ['foo', 'bar', 'baz'], got ${JSON.stringify(root.value)}`,
        );
    }

    commit();
}
