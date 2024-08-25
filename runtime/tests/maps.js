{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const object = {
    counters: new Map([["a", 0]]),
  };

  const count = object.counters.get("a");
  object.counters.set("a", count + 1);

  transaction.setRoot("test-root", object);
  transaction.commit();
}

{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const object = transaction.getRoot("test-root");

  // console.log(object);
  // console.log(object.counters.get("a"));

  if (!object) throw new Error("object should exist");
  // if (object.counters.size !== 1)
  //   throw new Error("object.counters should have size 1");
  if (object.counters.get("a") !== 1)
    throw new Error("object.counters.get('a') should be 1");

  transaction.commit();
}
