const baseArray = ["a", "b", "c", "d", "e"];

{
    const object = {
        value: [],
    };
    setRoot("test-root", object);
    commit();
}

{
    const object = getRoot("test-root");
    if (object.value.length !== 0) {
        throw new Error(`Expected length to be 0, got ${object.value.length}`);
    }

    object.value.push("a");
    commit();
}

{
    const object = getRoot("test-root");
    if (object.value.length !== 1) {
        throw new Error(`Expected length to be 1, got ${object.value.length}`);
    }
    if (object.value[0] !== "a") {
        throw new Error(`Expected "a", got ${object.value[0]}`);
    }

    object.value.push(...["b", "c", "d", "e"]);
    commit();
}

{
    const object = getRoot("test-root");
    if (object.value.length !== 5) {
        throw new Error(`Expected length to be 5, got ${object.value.length}`);
    }

    object.value.push("f");
    commit();
}

{
    const object = getRoot("test-root");
    if (object.value.length !== 6) {
        throw new Error(`Expected length to be 6, got ${object.value.length}`);
    }
    for (let i = 0; i < 5; i++) {
        if (object.value[i] !== baseArray[i]) {
            throw new Error(`Expected ${baseArray[i]}, got ${object.value[i]}`);
        }
    }
    if (object.value[5] !== "f") {
        throw new Error(`Expected "f", got ${object.value[5]}`);
    }
    commit();
}
