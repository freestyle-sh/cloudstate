// console.log(new Cloudstate("test-namespace").getTestsObject());

const cloudstate = new Cloudstate("test-namespace");

const object = {
  test: 5,
};

const object2 = cloudstate.getTestsObject();

console.log(object2);

// console.log(object2 === object);
// object.test = 6;
// console.log(object2.test);
