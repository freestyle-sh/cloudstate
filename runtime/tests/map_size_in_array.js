{
  const map = new Map();
  map.set("a", "alpha");

  const object = {
    value: [map],
  };

  setRoot("test-root", object);
  commit();
}

{
  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (!root.value) {
    throw new Error("root.value should exist");
  }
  if (root.value.length !== 1) {
    throw new Error(`root.value should have length 1`);
  }

  if (root.value[0].get("a") !== "alpha") {
    throw new Error(`root.value[0].get("a") should be "alpha"`);
  }

  if (root.value[0].size !== 1) {
    throw new Error(
      `root.value[0].size should be 1, got ${root.value[0].size}`,
    );
  }
}
