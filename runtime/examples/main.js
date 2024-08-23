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
// object.countersMap = new Map([["a", 1]]);
// console.log(object);
// console.log(object.countersMap.constructor.name);

// console.log(object.countersMap.get("a"));
// const currentCount = object.countersMap.get("a");
// object.countersMap.set("a", currentCount + 1);
// console.log(object.countersMap.get("a"));

transaction.setObject(object);
transaction.setRoot("test-root", object);
transaction.commit();
