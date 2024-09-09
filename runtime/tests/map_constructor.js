{
    const base = new Map();
    const root = {
        value: base,
    };
    setRoot("test-root", root);
    commit();
}

{
    const root = getRoot("test-root");
    if (!root) {
        throw new Error("No root found");
    }
    if (root.value === undefined) {
        throw new Error("No root.value found");
    }
    if (typeof root.value !== "object") {
        throw new Error("Map should be of primitive type object");
    }
    if (root.value.size === undefined) {
        throw new Error("Map should have the size property");
    }
    if (root.value.size !== 0) {
        throw new Error(`Expected 0 for map size, got ${root.value.size}`);
    }
}
