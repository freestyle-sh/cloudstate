{
    const base = [1, 2, 3, 4, 5];
    const object = {
        value: base,
    };

    setRoot("test-root", object);
    commit();
}

// END_FILE

{
    const expectedArr = [1, 2, 3, 4, 5];

    const object = getRoot("test-root");
    if (object.value.length !== expectedArr.length) {
        throw new Error(
            `Expected length to be ${expectedArr.length}, got ${object.value.length}`,
        );
    }

    const even = object.value.some((value) => value % 2 === 0);
    if (!even) {
        throw new Error(`Expected at least one even number in the array`);
    }

    const odd = object.value.some((value) => value % 2 !== 0);
    if (!odd) {
        throw new Error(`Expected at least one odd number in the array`);
    }

    const greaterThanFive = object.value.some((value) => value > 5);
    if (greaterThanFive) {
        throw new Error(`Expected no number greater than 5 in the array`);
    }

    const greaterThanZero = object.value.some((value) => value > 0);
    if (!greaterThanZero) {
        throw new Error(
            `Expected at least one number greater than 0 in the array`,
        );
    }
}
