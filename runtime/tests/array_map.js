const baseArray = [1, 4, 9, 16];
{
    const object = {
        value: baseArray,
    };
    setRoot("test-root", object);
    commit();
}
{
    const object = getRoot("test-root");
    const newArray = object.value.map((element) => element * 2);
    if (newArray.length !== 4) {
        throw new Error(`Expected length to be 4, got ${newArray.length}`);
    }
    for (let i = 0; i < newArray.length; i++) {
        if (newArray[i] !== baseArray[i] * 2) {
            throw new Error(
                `Expected ${JSON.stringify(baseArray[i] * 2)}, got ${
                    JSON.stringify(
                        newArray[i],
                    )
                }`,
            );
        }
    }
}
