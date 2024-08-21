// console.log(new Cloudstate("test-namespace").getTestsObject());

const cloudstate = new Cloudstate("test-namespace");

const object = {
  test: 5,
};

const object2 = cloudstate.getTestsObject(object);

console.log(object2);