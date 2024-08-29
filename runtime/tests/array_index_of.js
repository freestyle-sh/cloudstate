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
    const idx1 = object.value.indexOf(12); // value: 1
    if (idx1 !== 1) {
        throw new Error(`Expected 1, got ${idx1}`);
    }
    const idx2 = object.value.indexOf(200); // value: -1
    if (idx2 !== -1) {
        throw new Error(`Expected -1, got ${idx2}`);
    }
    const idx3 = object.value.indexOf(130); // value: 3
    if (idx3 !== 3) {
        throw new Error(`Expected 3, got ${idx3}`);
    }
    const idx4 = object.value.indexOf(44); // value: 4
    if (idx4 !== 4) {
        throw new Error(`Expected 4, got ${idx4}`);
    }
    const idx5 = object.value.indexOf(5); // value: 0
    if (idx5 !== 0) {
        throw new Error(`Expected 0, got ${idx5}`);
    }
}
