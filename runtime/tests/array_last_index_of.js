const baseArray = [5, 12, 8, 130, 44, 5];

{
    const object = {
        value: baseArray,
    };
    setRoot("test-root", object);
    commit();
}

{
    const object = getRoot("test-root");
    const idx1 = object.value.lastIndexOf(12);
    if (idx1 !== 1) {
        throw new Error(`Expected 1, got ${idx1}`);
    }
    const idx2 = object.value.lastIndexOf(200);
    if (idx2 !== -1) {
        throw new Error(`Expected -1, got ${idx2}`);
    }
    const idx3 = object.value.lastIndexOf(130);
    if (idx3 !== 3) {
        throw new Error(`Expected 3, got ${idx3}`);
    }
    const idx4 = object.value.lastIndexOf(44);
    if (idx4 !== 4) {
        throw new Error(`Expected 4, got ${idx4}`);
    }
    const idx5 = object.value.lastIndexOf(5);
    if (idx5 !== 5) {
        throw new Error(`Expected 5, got ${idx5}`);
    }
}
