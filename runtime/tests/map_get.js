{
    const map = new Map([
        ["foo", 1],
        ["bar", 2],
        ["baz", 3],
    ]);

    const object = {
        value: map,
    };

    setRoot("test-root", object);
    commit();
}

{
    const object = getRoot("test-root");

    // keys exist
    const a = object.value.get("foo");
    if (a !== 1) {
        throw new Error(
            `Expected ${JSON.stringify(1)}, got ${JSON.stringify(a)}`,
        );
    }
    const b = object.value.get("bar");
    if (b !== 2) {
        throw new Error(
            `Expected ${JSON.stringify(2)}, got ${JSON.stringify(b)}`,
        );
    }
    const c = object.value.get("baz");
    if (c !== 3) {
        throw new Error(
            `Expected ${JSON.stringify(3)}, got ${JSON.stringify(c)}`,
        );
    }

    // keys do not exist
    console.log("get(qux)", "qux");
    if (object.value.get("qux") !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get("qux"))
            }`,
        );
    }
    console.log("get(undefined)", undefined);
    if (object.value.get(0) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(undefined))
            }`,
        );
    }
    console.log("get(null)", null);
    if (object.value.get(null) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(null))
            }`,
        );
    }
    console.log("get(0)", 0);
    if (object.value.get(0) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(0))
            }`,
        );
    }
    console.log("get(false)", false);
    if (object.value.get(false) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(false))
            }`,
        );
    }
    console.log('get("")', "");
    if (object.value.get("") !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(""))
            }`,
        );
    }
    console.log("get(Symbol())", Symbol());
    if (object.value.get(Symbol()) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(Symbol()))
            }`,
        );
    }
    console.log("get({})", {});
    if (object.value.get({}) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get({}))
            }`,
        );
    }
    console.log("get([])", []);
    if (object.value.get([]) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get([]))
            }`,
        );
    }
    console.log("get(() => {})", () => {});
    if (object.value.get(() => {}) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(() => {}))
            }`,
        );
    }
    console.log("get(object)", object);
    if (object.value.get(object) !== undefined) {
        throw new Error(
            `Expected ${JSON.stringify(undefined)}, got ${
                JSON.stringify(object.value.get(object))
            }`,
        );
    }
}
