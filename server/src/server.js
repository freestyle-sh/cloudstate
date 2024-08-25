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
        const root = new klass();
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
  object.addItem("test");
  transaction.commit();
}

{
  const cloudstate = new Cloudstate("default", {
    customClasses: Object.keys(classes).map((key) => classes[key]),
  });

  const transaction = cloudstate.createTransaction();

  const object = transaction.getRoot("todo-list");

  if (!object.getItems()[0].text === "test") {
    throw new Error("Item was not added to todo list");
  }

  transaction.commit();
}
