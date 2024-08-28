const base = new Map();

{
    const object = {
        value: base,
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
    if (root.value.size !== 0) {
        throw new Error(
            `root.value should have size 0, got ${root.value.size}`,
        );
    }

    root.value.set("a", 1);
    if (root.value.size !== 1) {
        throw new Error(
            `root.value should have size 1, got ${root.value.size}`,
        );
    }
    if (root.value.get("a") !== 1) {
        throw new Error(
            `root.value.get('a') should be 1, got ${root.value.get("a")}`,
        );
    }

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
    if (root.value.size !== 1) {
        throw new Error(
            `root.value should have size 1, got ${root.value.size}`,
        );
    }
    if (root.value.get("a") !== 1) {
        throw new Error(
            `root.value.get('a') should be 1, got ${root.value.get("a")}`,
        );
    }
}
