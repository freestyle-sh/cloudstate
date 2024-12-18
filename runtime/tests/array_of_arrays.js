{
    const root = {
        arrs: [
            [{ num: 0 }, { num: 1 }],
        ],
    };

    setRoot("test-root", root);
}

// END_FILE

{
    const root = getRoot("test-root");
    if (root.arrs.length !== 1) {
        throw new Error("Expected root.arrs.length to be 1");
    }

    console.log(root.arrs[0][0]);

    if (root.arrs[0][0].num !== 0) {
        throw new Error("Expected root.arrs[0][0].num to be 0");
    }

    if (root.arrs[0][1].num !== 1) {
        throw new Error("Expected root.arrs[0][1].num to be 1");
    }
}

// TODO: more thourougly test array of arrays
