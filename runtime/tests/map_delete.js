const baseMap = new Map([
  ["a", "alpha"],
  ["b", "beta"],
  ["c", "charlie"],
  ["d", "delta"],
  ["e", "echo"],
]);
{
  const object = {
    value: baseMap,
  };

  setRoot("test-root", object);
  commit();
}

{
  const object = getRoot("test-root");
  const a = object.value.get("a");
  if (a !== "alpha") {
    throw new Error(
      `Expected ${JSON.stringify("alpha")}, got ${JSON.stringify(a)}`
    );
  }
  if (!object.value.has("a")) {
    throw new Error(`Expected ${JSON.stringify("a")} to exist`);
  }

  const deletedVal = object.value.delete("a");
  if (!deletedVal) {
    throw new Error(
      `Expected ${JSON.stringify(
        "a"
      )} to be deleted (return value should be truthy)`
    );
  }

  commit();
}

{
  const object = getRoot("test-root");
  if (object.value.has("a")) {
    throw new Error(`Expected ${JSON.stringify("a")} to be deleted`);
  }

  commit();
}

{
  const object = getRoot("test-root");
  const values = object.value.values();
  const arr = Array.from(values);

  // check that a was deleted
  if (object.value.has("a")) {
    throw new Error(`Expected ${JSON.stringify("alpha")} to not exist`);
  }

  if (arr.length !== Array.from(baseMap.values()).length - 1) {
    throw new Error(
      `Should have been arr.length = ${
        Array.from(baseMap.values()).length - 1
      }, got ${arr.length}`
    );
  }

  commit();
}

{
  const object = getRoot("test-root");

  // delete key that doesn't exist
  const deletedVal = object.value.delete("z");
  if (deletedVal) {
    throw new Error(`Expected ${JSON.stringify("z")} to not exist`);
  }

  commit();
}
