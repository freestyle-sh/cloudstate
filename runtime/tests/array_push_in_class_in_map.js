{
    class Auth {
        users = new Map();
    }

    registerCustomClass(Auth);

    setRoot("test-root", new Auth());
}

// END_FILE

{
    class Auth {
        users = new Map();
    }

    class Cases {
        cases = [];
        createCase() {
            this.cases.push({
                name: "test",
            });
        }
    }

    class User {
        cases = new Cases();
        blob = new Blob(["hello"], {
            type: "text/plain",
        });
        blobs = [new Blob(["hello"])];
    }

    registerCustomClass(Auth);
    registerCustomClass(Cases);
    registerCustomClass(User);

    const object = getRoot("test-root");

    object.users.set("tiko", new User());
}

// END_FILE

{
    class Auth {
        users = new Map();
    }

    class Cases {
        cases = [];
        createCase() {
            this.cases.push({
                name: "test",
            });
        }
    }

    class User {
        cases = new Cases();
        blob = new Blob(["hello"], {
            type: "text/plain",
        });
    }

    registerCustomClass(Auth);
    registerCustomClass(Cases);
    registerCustomClass(User);

    const object = getRoot("test-root");

    object.users.get("tiko").cases.createCase();
}

// END_FILE

{
    class Auth {
        users = new Map();
    }

    class Cases {
        cases = [];
        createCase() {
            this.cases.push({
                name: "test",
            });
        }
    }

    class User {
        cases = new Cases();
        blob = new Blob(["hello"], {
            type: "text/plain",
        });
    }

    registerCustomClass(Auth);
    registerCustomClass(Cases);
    registerCustomClass(User);

    const object = getRoot("test-root");

    if (object.users.get("tiko").cases.cases.length !== 1) {
        throw new Error(
            `Expected object.users.get("tiko").cases.cases.length to be 1. Got ${
                object.users.get("tiko").cases.cases.length
            }`,
        );
    }

    if (object.users.get("tiko").cases.cases[0].name !== "test") {
        throw new Error(
            `Expected object.users.get("tiko").cases.cases[0].name to be "test". Got ${
                object.users.get("tiko").cases.cases[0].name
            }`,
        );
    }
}
