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
  object.value.set("f", "foxtrot");
  object.value.set("object", { a: 1, b: 2, c: 3 });

  commit();
}

{
  const object = getRoot("test-root");
  const values = object.value.values();
  const arr = Array.from(values);
  const targetLength = 7;
  if (arr.length !== targetLength) {
    throw new Error(
      `Should have been arr.length = ${targetLength}, got ${arr.length}`
    );
  }
}
