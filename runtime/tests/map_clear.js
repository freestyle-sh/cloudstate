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

  // verify map is set correctly
  const a = object.value.get("foo");
  if (a !== 1) {
    throw new Error(`Expected ${JSON.stringify(1)}, got ${JSON.stringify(a)}`);
  }
  if (!object.value.has("foo")) {
    throw new Error(`Expected ${JSON.stringify("foo")} to exist`);
  }
  if (object.value.size !== 3) {
    throw new Error(`Expected size to be 3, got ${object.value.size}`);
  }

  // clear map
  object.value.clear();

  // verify map is cleared
  if (object.value.size !== 0) {
    throw new Error(`Expected size to be 0, got ${object.value.size}`);
  }
  if (!object.value.has("foo")) {
    throw new Error(`Expected ${JSON.stringify("foo")} to be deleted`);
  }

  commit();
}

{
  const object = getRoot("test-root");

  if (object.value.size !== 0) {
    throw new Error(`Expected size to be 0, got ${object.value.size}`);
  }

  if (object.value.has("foo")) {
    throw new Error(`Expected ${JSON.stringify("foo")} to be deleted`);
  }

  commit();
}
