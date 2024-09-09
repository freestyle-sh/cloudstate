{
    const map = new Map([
        ["foo", 1],
        ["bar", 2],
        ["baz", 3],
    ]);

    const object = {
        value: map,
    };

    setRoot("test-root", object);
    commit();
}

// END_FILE

{
    const object = getRoot("test-root");

    // keys exist
    if (!object.value.has("foo")) {
        throw new Error(`Expected ${JSON.stringify("foo")} to exist`);
    }
    if (!object.value.has("bar")) {
        throw new Error(`Expected ${JSON.stringify("bar")} to exist`);
    }
    if (!object.value.has("baz")) {
        throw new Error(`Expected ${JSON.stringify("baz")} to exist`);
    }

    // keys do not exist
    if (object.value.has("qux")) {
        throw new Error(`Expected ${JSON.stringify("qux")} to not exist`);
    }
    if (object.value.has(undefined)) {
        throw new Error(`Expected ${JSON.stringify(undefined)} to not exist`);
    }
    if (object.value.has(null)) {
        throw new Error(`Expected ${JSON.stringify(null)} to not exist`);
    }
    if (object.value.has(0)) {
        throw new Error(`Expected ${JSON.stringify(0)} to not exist`);
    }
    if (object.value.has(false)) {
        throw new Error(`Expected ${JSON.stringify(false)} to not exist`);
    }
    if (object.value.has("")) {
        throw new Error(`Expected ${JSON.stringify("")} to not exist`);
    }
    if (object.value.has(Symbol())) {
        throw new Error(`Expected ${JSON.stringify(Symbol())} to not exist`);
    }
    if (object.value.has({})) {
        throw new Error(`Expected ${JSON.stringify({})} to not exist`);
    }
    if (object.value.has([])) {
        throw new Error(`Expected ${JSON.stringify([])} to not exist`);
    }
    if (object.value.has(() => {})) {
        throw new Error(`Expected ${JSON.stringify(() => {})} to not exist`);
    }
    if (object.value.has(object)) {
        throw new Error(`Expected ${JSON.stringify(object)} to not exist`);
    }
}
