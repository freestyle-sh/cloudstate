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
    const idx1 = object.value.findLastIndex((element) => element > 10); // value: 44
    if (idx1 !== 4) {
        throw new Error(`Expected 4, got ${idx1}`);
    }
    const idx2 = object.value.findLastIndex((element) => element > 200); // value: undefined
    if (idx2 !== -1) {
        throw new Error(`Expected -1, got ${idx2}`);
    }
    const idx3 = object.value.findLastIndex((element) => element <= 130); // value: 44
    if (idx3 !== 5) {
        throw new Error(`Expected 5, got ${idx3}`);
    }
    const idx4 = object.value.findLastIndex((element) =>
        element > 15 && element < 100
    ); // value: 44
    if (idx4 !== 4) {
        throw new Error(`Expected 4, got ${idx4}`);
    }
    const idx5 = object.value.findLastIndex((element) => element === 5); // value: 5
    if (idx5 !== 5) {
        throw new Error(`Expected 5, got ${idx5}`);
    }
}
