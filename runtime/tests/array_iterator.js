const base = [1, 2, 3, "a", "b", "c"];
{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();
  const object = {
    value: [...base],
  };
  console.log("transaction .setRoot");
  transaction.setRoot("test-root", object);
  console.log("transaction .commit");
  transaction.commit();
  console.log("Post transaction .commit");
}
{
  // test for of loop
  const transaction = cloudstate.createTransaction();

  const root = transaction.getRoot("test-root");

  if (!root) throw new Error("root should exist");

  if (!root.value) throw new Error("root.value should exist");

  if (root.value.length !== 4)
    throw new Error("root.value should have length 4");

  let value = [...base];
  let i = 0;
  console.log("pre for of loop");
  for (const v of root.value) {
    console.log("value", v);
    if (v !== value[i]) {
      throw new Error("value does not match");
    }
  }

  console.log("post for of loop");
  if (i !== value.length) {
    throw new Error("value iterator does not match");
  }
}
