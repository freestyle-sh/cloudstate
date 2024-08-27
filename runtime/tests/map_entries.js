const base = new Map([
  ["a", 1],
  ["b", 2],
  ["c", 3],
  ["d", "a"],
  ["e", "b"],
  ["f", "c"],
]);
// base.size = 4;
const base_values = [...base.entries()];
{
  const object = {
    value: base,
  };

  setRoot("test-root", object);
  commit();
}

{
  const root = getRoot("test-root");

  if (!root) throw new Error("root should exist");

  if (!root.value) throw new Error("root.value should exist");

  let i = 0;
  // const values =
  try {
    const v = [...root.value.entries()];
  } catch (e) {
    console.log("ERR", e);
  }
  for (const v of root.value.entries()) {
    //todo: DEEP EQUAL
    if (v[0] !== base_values[i][0] || v[1] !== base_values[i][1]) {
      throw new Error(
        `value mismatch at index ${i}: ${v} !== ${base_values[i]}`
      );
    }
    i++;
  }

  if (i !== base_values.length) {
    throw new Error("iterator length does not match");
  }
}
