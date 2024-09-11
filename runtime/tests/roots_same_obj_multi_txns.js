{
  const object = {
    value: 32,
  };

  setRoot("root1", object);

  commit();

  setRoot("root2", object);
}

// END_FILE

{
  // Check if the value is the same
  const root1 = getRoot("root1");
  const root2 = getRoot("root2");

  if (!root1) {
    throw new Error("Root 1 should exist");
  }
  if (!root2) {
    throw new Error("Root 2 should exist");
  }
  if (root1.value !== root2.value) {
    throw new Error("Values are not the same");
  }
  if (root1 !== root2) {
    throw new Error("Roots are not the same");
  }
}
