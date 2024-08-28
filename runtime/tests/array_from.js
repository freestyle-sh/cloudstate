// Initialize data
const arrayLike = { 0: "a", 1: "b", 2: "c", length: 3 };
const mapFn = (x) => x.toUpperCase();

{
    const object = {
        objValue: arrayLike,
    };

    setRoot("test-root", object);
    commit();
}

{
    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (!root.objValue) {
        throw new Error("root.value should exist");
    }
    if (root.objValue.length !== 3) {
        throw new Error("root.value should have length 3");
    }

    const array = Array.from(root.objValue, mapFn);
    if (array.length !== 3) {
        throw new Error("array should have length 3");
    }
    if (array[0] !== "A") {
        throw new Error(`array[0] should be "A", got ${array[0]}`);
    }
    if (array[1] !== "B") {
        throw new Error(`array[1] should be "B", got ${array[1]}`);
    }
    if (array[2] !== "C") {
        throw new Error(`array[2] should be "C", got ${array[2]}`);
    }

    root.arrValue = array;
    commit();
}

{
    const root = getRoot("test-root");
    if (!root) {
        throw new Error("root should exist");
    }
    if (!root.arrValue) {
        throw new Error("root.arrValue should exist");
    }
    if (root.arrValue.length !== 3) {
        throw new Error("root.arrValue should have length 3");
    }
    if (root.arrValue[0] !== "A") {
        throw new Error(
            `root.arrValue[0] should be "A", got ${root.arrValue[0]}`,
        );
    }
    if (root.arrValue[1] !== "B") {
        throw new Error(
            `root.arrValue[1] should be "B", got ${root.arrValue[1]}`,
        );
    }
    if (root.arrValue[2] !== "C") {
        throw new Error(
            `root.arrValue[2] should be "C", got ${root.arrValue[2]}`,
        );
    }
}
