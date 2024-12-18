{
    const root = {
        arr: [
            {
                duration: 250,
            },
            {
                duration: 500,
            },
        ],
    };

    setRoot("test-root", root);
}

// END_FILE

{
    const root = getRoot("test-root");

    const promises = root.arr.map((item) => {
        return new Promise((resolve) => {
            setTimeout(() => {
                resolve(item.duration);
            }, item.duration);
        });
    });

    const resolved = await Promise.all(promises);

    if (resolved[0] !== 250) {
        throw new Error("Expected resolved[0] to be 250");
    }

    if (resolved[1] !== 500) {
        throw new Error("Expected resolved[1] to be 500");
    }
}
