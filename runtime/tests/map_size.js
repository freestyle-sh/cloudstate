const base = new Map([
    ["a", 1],
    ["b", 2],
    ["c", 3],
    ["d", "a"],
    ["e", "b"],
    ["f", "c"],
])
const baseSize = base.size;
{

  const object = {
    value: base
  };

  setRoot("test-root", object);
  commit();

}


{
  const root = getRoot("test-root");

    if (!root) throw new Error("root should exist");

    if (!root.value) throw new Error("root.value should exist");

    console.log("root.value.size", root.value.size);
    if (root.value.size !== baseSize)
      throw new Error(`root.value should have size ${baseSize}, got ${root.value.size}`);
}
