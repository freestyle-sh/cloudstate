{
    const object = {
        value: [{ a: 1 }],
    };

    setRoot("test-root", object);
}

// END_FILE

{
    const object = getRoot("test-root");

    if (object.value[0].a !== 1) {
        throw new Error(`Expected 1, got ${object.value[0].a}`);
    }
}
