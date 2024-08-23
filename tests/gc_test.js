{
  // Test that the garbage collector works correctly
  const cloudstate = new Cloudstate("test-namespace");

  const root = {};

  const nested1 = {
    value: 5,
    nestedObject: {
      value: 6,
    },
  };

  root.nested1 = nested1;

  const nested2 = {
    value: 7,
    otherNest: {
      value: 8,
    },
  };

  root.nested2 = nested2;

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

  if (!root) throw new Error("root should exist");

  if (!root.nested1) throw new Error("root.nested1 should exist");

  if (!root.nested1.nestedObject)
    throw new Error("root.nested1.nestedObject should exist");

  if (!root.nested2) throw new Error("root.nested2 should exist");

  if (!root.nested2.otherNest)
    throw new Error("root.nested2.otherNest should exist");

  transaction.commit();
}

{
  // delete the nested1
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const root = transaction.getRoot("test-root");

  if (!root) throw new Error("root should exist");

  delete root.nested1;

  transaction.setObject(root);
  transaction.setRoot("test-root", root);

  transaction.commit();
}

{
  // confirm nested1 is gone but nested2 is still there

  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const root = transaction.getRoot("test-root");

  if (!root) throw new Error("root should exist");

  if (root.nested1) throw new Error("root.nested1 should not exist");

  if (!root.nested2) throw new Error("root.nested2 should exist");

  if (!root.nested2.otherNest)
    throw new Error("root.nested2.otherNest should exist");

  transaction.commit();
}
