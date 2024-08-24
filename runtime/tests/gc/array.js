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

  transaction.setObject(root);

  transaction.setRoot("test-root", root);

  transaction.commit();
  
  console.log("A");

  const transaction2 = cloudstate.createTransaction();


  const root2 = transaction2.getRoot("test-root");


  console.log("B", root2.nested.value[3]);

  // transaction2.setRoot("test-root", root2);
  try {
    transaction2.setObject(root2);

  } catch (error) {
    console.log("ERRO asfsafasfßR", error);
  }

  console.log("C");

  transaction2.commit();

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
