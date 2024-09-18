{
    setRoot("test-root", {
        value: [],
    });
}

// END_FILE

{
    const object = getRoot("test-root");

    object.value.push({
        a: 1,
    });
}

// END_FILE

{
    const object = getRoot("test-root");

    if (object.value.length !== 1) {
        throw new Error(
            `Expected object.value.length to be 1. Got ${object.value.length}`,
        );
    }

    if (object.value[0].a !== 1) {
        throw new Error(
            `Expected object.value[0].a to be 1. Got ${object.value[0].a}`,
        );
    }
}
