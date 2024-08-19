const cloudstate = new Cloudstate("test-namespace");

const transaction = cloudstate.createTransaction();

const object = transaction.getRoot("test-root") || {
  counters: [
    {
      count: 0,
    },
  ],
};

object.counters[0].count += 1;
console.log(object);

transaction.setObject(object);
transaction.setRoot("test-root", object);
transaction.commit();
