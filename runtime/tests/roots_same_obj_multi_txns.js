const object = {
    value: 32,
};

{
    setRoot("root1", object);
    commit();
}

{
    setRoot("root2", object);
    commit();
}

{
    // Check if the value is the same
    const root1 = getRoot("root1");
    const root2 = getRoot("root2");

    if (root1 !== root2) {
        throw new Error("Roots are not the same");
    }
}
