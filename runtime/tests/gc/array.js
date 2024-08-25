const cloudstate = new Cloudstate("test-namespace");


{
  // Test that the garbage collector works correctly

  const root = {
    nested: {
      value: [1, 2, 3, 4],
      value2:[5, 6, 7, 8, 9],
    },
  };

  const transaction = cloudstate.createTransaction();

  transaction.setRoot("test-root", root);

  transaction.commit();
  
}

{

  const transaction = cloudstate.createTransaction();

  const root = transaction.getRoot("test-root");

  if (!root) throw new Error("root should exist");

  if (!root.nested) throw new Error("root.nested should exist");

  if (!root.nested.value) throw new Error("root.nested.value should exist");

  if (!root.nested.value2) throw new Error("root.nested.value2 should exist");

  if (root.nested.value.length !== 4) throw new Error("root.nested.value should have length 4");

  if (root.nested.value2.length !== 5) throw new Error("root.nested.value2 should have length 5");

  transaction.commit();
}

{
  // delete the nested1

  const transaction = cloudstate.createTransaction();

  const root = transaction.getRoot("test-root");

  if (!root) throw new Error("root should exist");

  delete root.nested.value;
  transaction.commit();


}

// {
//   // delete the nested1
//   const transaction = cloudstate.createTransaction();

//   const root = transaction.getRoot("test-root");

//   if (!root) throw new Error("root should exist");

//   delete root.nested.value;

//   console.log("A1");
//   transaction.setObject(root);
//   console.log("A2");
//   transaction.setRoot("test-root", root);
//   console.log("A3");
//   transaction.commit();
//   console.log("A4");
// }

// {
//   const cloudstate = new Cloudstate("test-namespace");

//   const transaction = cloudstate.createTransaction();

//   const root = transaction.getRoot("test-root");

//   console.log("NEW ROOT", root);

//   if (!root) throw new Error("root should exist");

//   if (!root.value) throw new Error("root.value should exist");

//   if (!root.value2) throw new Error("root.value2 should exist");

//   transaction.commit();
// }
