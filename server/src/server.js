import * as classes from "./lib.js";

{
  const cloudstate = new Cloudstate("default", {
    customClasses: Object.values(classes),
  });

  const transaction = cloudstate.createTransaction();

  for (const className of Object.keys(classes)) {
    const klass = classes[className];
    if (klass.id) {
      const object = transaction.getRoot(klass.id);
      if (!object) {
        console.log("Constructing static class", className);
        console.log("Creating root", klass.id);
        const root = new klass();
        transaction.setObject(root);
        transaction.setRoot(klass.id, root);
      }
    }
  }

  transaction.commit();
}

{
  const cloudstate = new Cloudstate("default", {
    customClasses: Object.keys(classes).map((key) => classes[key]),
  });

  const transaction = cloudstate.createTransaction();

  const object = transaction.getRoot("todo-list");
  // console.log(object);
  // console.log(object);
  // object.addItem("test");
}
