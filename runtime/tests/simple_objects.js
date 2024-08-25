{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const object = {
    count: 1,
  };

  transaction.setRoot("test-root", object);

  transaction.commit();
}

{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const object = transaction.getRoot("test-root");

  if (!object) throw new Error("object should exist");
  if (object.count !== 1) throw new Error("object.count should be 1");
}
