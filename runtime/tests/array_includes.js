const base = ["a", "b", "c", "d", "e", "f"];
{
  const object = {
    value: [...base],
  };

  setRoot("test-root", object);

  commit();
}
// check includes
{
  const root = getRoot("test-root");

  if (!root) throw new Error("root should exist");

  if (!root.value) throw new Error("root.value should exist");

  if (root.value.length !== base.length)
    throw new Error(`root.value should have length ${base.length}`);

  if (!root.value.includes("a"))
    throw new Error(`root.value should include "a"`);

  if (!root.value.includes("f"))
    throw new Error(`root.value should include "f"`);

  if (root.value.includes("z"))
    throw new Error(`root.value should not include "z"`);
}
