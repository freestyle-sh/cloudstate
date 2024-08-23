{
  // Test that the garbage collector works correctly
  const cloudstate = new Cloudstate("test-namespace");

  const root = {
    value: [1, 2, 3, 4, 5],
    value2: [6, 7, 8, 9],
  };


  const transaction = cloudstate.createTransaction();

  transaction.setObject(root);

  transaction.setRoot("test-root", root);

  transaction.commit();
}

{
  // confirm that the object is still there
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const root = transaction.getRoot("test-root");

  console.log("root ARR TYPE", root.value instanceof Array, root.value[0]);

  if (!root) throw new Error("root should exist");

  if (!root.value) throw new Error("root.value should exist");

  if (!root.value2) throw new Error("root.value2 should exist");

  if (root.value.length !== 5) throw new Error("root.value.length should be 5");

  if (root.value2.length !== 4) throw new Error("root.value2.length should be 4");

  transaction.commit();
}

{
  // delete the nested1
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const root = transaction.getRoot("test-root");

  if (!root) throw new Error("root should exist");

  console.log("root ARR TYPE", root.value instanceof Array, root.value[0]);

  transaction.commit();
}

{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const root = transaction.getRoot("test-root");

  if (!root) throw new Error("root should exist");

  if (!root.value) throw new Error("root.value should exist");

  if (!root.value2) throw new Error("root.value2 should exist");
  
  transaction.commit();
}
