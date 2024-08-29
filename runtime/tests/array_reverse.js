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

    object.value.reverse();
    commit();
}

{
    const object = getRoot("test-root");
    if (object.value.length !== 0) {
        throw new Error(`Expected length to be 0, got ${object.value.length}`);
    }

    object.value.push(...baseArray);
    object.value.reverse();
    commit();
}

{
    const object = getRoot("test-root");
    if (object.value.length !== 5) {
        throw new Error(`Expected length to be 5, got ${object.value.length}`);
    }
    for (let i = 0; i < baseArray.length; i++) {
        if (object.value[i] === undefined) {
            throw new Error(
                `Expected array item ${baseArray[i]}, got undefined`,
            );
        }
        if (object.value[i] !== baseArray[baseArray.length - 1 - i]) {
            throw new Error(
                `Expected array item ${baseArray[i]}, got ${object.value[i]}}`,
            );
        }
    }
}
