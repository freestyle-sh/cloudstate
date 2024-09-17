{
    class Arr {
        arr = [];
        constructor() {
            this.value = [];
        }

        add() {
            this.arr.push({
                a: 1,
            });
        }
    }
    registerCustomClass(Arr);

    setRoot("test-root", new Arr());
}

// END_FILE

{
    class Arr {
        arr = [];
        constructor() {
            this.value = [];
        }

        add() {
            this.arr.push({
                a: 1,
            });
        }
    }
    registerCustomClass(Arr);

    const object = getRoot("test-root");

    object.add();
}

// END_FILE

{
    class Arr {
        arr = [];
        constructor() {
            this.value = [];
        }

        add() {
            this.arr.push({
                a: 1,
            });
        }
    }
    registerCustomClass(Arr);

    const object = getRoot("test-root");

    if (object.arr.length !== 1) {
        throw new Error(
            `Expected object.arr.length to be 1. Got ${object.arr.length}`,
        );
    }

    if (object.arr[0].a !== 1) {
        throw new Error(
            `Expected object.arr[0].a to be 1. Got ${object.arr[0].a}`,
        );
    }
}
