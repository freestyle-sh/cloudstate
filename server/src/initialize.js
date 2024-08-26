import * as classes from "./lib.js";
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
