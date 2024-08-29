const baseArray = ["a", "b", "c", "d", "e"];

{
    const object = {
        value: baseArray,
    };
    setRoot("test-root", object);
}

{
    const object = getRoot("test-root");
    if (object.value.length !== 5) {
        throw new Error(`Expected length to be 5, got ${object.value.length}`);
    }

    const popped = object.value.pop();
    if (popped !== "e") {
        throw new Error(`Expected "e", got ${popped}`);
    }
    console.log("Popped 1", popped);
    console.log("Array", object.value);
    commit();
}
{
    const object = getRoot("test-root");
    // console.log("4======", object.value[4]);
    if (object.value.length !== 4) {
        throw new Error(`Expected length to be 4, got ${object.value.length}`);
    }
    if (object.value[0] !== "a") {
        throw new Error(`Expected "a", got ${object.value[0]}`);
    }
    if (object.value[1] !== "b") {
        throw new Error(`Expected "b", got ${object.value[1]}`);
    }
    if (object.value[2] !== "c") {
        throw new Error(`Expected "c", got ${object.value[2]}`);
    }
    if (object.value[3] !== "d") {
        throw new Error(`Expected "d", got ${object.value[3]}`);
    }

    const popped = object.value.pop();
    if (popped !== "d") {
        throw new Error(`Expected "d", got ${popped}`);
    }
    console.log("Popped 2", popped);
    console.log("Array", object.value);
    commit();
}
{
    const object = getRoot("test-root");
    if (object.value.length !== 3) {
        throw new Error(`Expected length to be 3, got ${object.value.length}`);
    }
    if (object.value[0] !== "a") {
        throw new Error(`Expected "a", got ${object.value[0]}`);
    }
    if (object.value[1] !== "b") {
        throw new Error(`Expected "b", got ${object.value[1]}`);
    }
    if (object.value[2] !== "c") {
        throw new Error(`Expected "c", got ${object.value[2]}`);
    }

    const popped = object.value.pop();
    if (popped !== "c") {
        throw new Error(`Expected "c", got ${popped}`);
    }
    commit();
}
{
    const object = getRoot("test-root");
    if (object.value.length !== 2) {
        throw new Error(`Expected length to be 2, got ${object.value.length}`);
    }
    if (object.value[0] !== "a") {
        throw new Error(`Expected "a", got ${object.value[0]}`);
    }
    if (object.value[1] !== "b") {
        throw new Error(`Expected "b", got ${object.value[1]}`);
    }

    const popped = object.value.pop();
    if (popped !== "b") {
        throw new Error(`Expected "b", got ${popped}`);
    }
    commit();
}
{
    const object = getRoot("test-root");
    if (object.value.length !== 1) {
        throw new Error(`Expected length to be 1, got ${object.value.length}`);
    }
    if (object.value[0] !== "a") {
        throw new Error(`Expected "a", got ${object.value[0]}`);
    }

    const popped = object.value.pop();
    if (popped !== "a") {
        throw new Error(`Expected "a", got ${popped}`);
    }
    commit();
}
{
    const object = getRoot("test-root");
    if (object.value.length !== 0) {
        throw new Error(`Expected length to be 0, got ${object.value.length}`);
    }

    const popped = object.value.pop();
    if (popped !== undefined) {
        throw new Error(`Expected undefined, got ${popped}`);
    }
}
