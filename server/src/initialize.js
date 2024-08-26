import * as classes from "./lib.js";
globalThis.cloudstate.customClasses = Object.keys(classes).map(
  (key) => classes[key]
);

for (const className of Object.keys(classes)) {
  const klass = classes[className];
  if (klass.id) {
    const object = getRoot(klass.id);
    if (!object) {
      const root = new klass();
      setRoot(klass.id, root);
    }
  }
}
