{
  const object = {
    value: new Map([
      ["a", {}],
      ["b", {}],
    ]),
  };

  setRoot("test-root", object);
}

commit();

{
  const object = getRoot("test-root");
  const arr = Array.from(object.value.values());

  if (arr.length !== 2) {
    throw new Error("Array.from(object.value.values()) should have length 2");
  }
}
