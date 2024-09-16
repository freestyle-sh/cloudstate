{
    setRoot("test-root", {
        value: [
            new Map([["a", 1]]),
        ],
    });
}

// END_FILE

{
    // check that the map is saved correctly
    const object = getRoot("test-root");
    if (!(object.value[0] instanceof Map)) {
        throw new Error(
            `Expected object.value[0] to be saved as a Map. Got ${
                object.value[0]
            }`,
        );
    }

    if (object.value[0].size !== 1) {
        throw new Error(
            `Expected object.value[0].size to be 1. Got ${
                object.value[0].size
            }`,
        );
    }

    if (object.value[0].get("a") !== 1) {
        throw new Error(
            `Expected object.value[0].get("a") to be 1. Got ${
                object.value[0].get("a")
            }`,
        );
    }

    // update the map
    object.value[0].set("b", 2);
}

// END_FILE

{
    // check that the map is updated correctly
    const object = getRoot("test-root");
    if (!(object.value[0] instanceof Map)) {
        throw new Error(
            `Expected object.value[0] to be saved as a Map. Got ${
                object.value[0]
            }`,
        );
    }
    if (object.value[0].size !== 2) {
        throw new Error(
            `Expected object.value[0].size to be 1. Got ${
                object.value[0].size
            }`,
        );
    }
    if (object.value[0].get("a") !== 1) {
        throw new Error(
            `Expected object.value[0].get("a") to be 1. Got ${
                object.value[0].get("a")
            }`,
        );
    }
    if (object.value[0].get("b") !== 2) {
        throw new Error(
            `Expected object.value[0].get("b") to be 2. Got ${
                object.value[0].get("b")
            }`,
        );
    }
}
