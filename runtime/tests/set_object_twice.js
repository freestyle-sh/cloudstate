{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const object = {
    // counters: [0],
    map: new Map([["a", 1]]),
  };

  transaction.setObject(object);
  transaction.setObject(object);

  transaction.setRoot("test-root", object);

  transaction.commit();
}
