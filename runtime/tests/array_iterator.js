const base = [1, 2, 3, "a", "b", "c"];
{

  const object = {
    value: [...base],
  };

  setRoot("test-root", object);
  commit();

}


{
  const root = getRoot("test-root");

  if (!root) throw new Error("root should exist");

  if (!root.value) throw new Error("root.value should exist");

  if (root.value.length !== 4)
    throw new Error("root.value should have length 4");

  let value = [...base];
  let i = 0;
  for (const v of root.value) {
    if (v !== value[i]) {
      throw new Error("value does not match");
    }
  }

  console.log("post for of loop");
  if (i !== value.length) {
    throw new Error("value iterator does not match");
  }

  commit();
}
