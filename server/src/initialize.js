import * as classes from "./lib.js";
globalThis.cloudstate.customClasses = Object.keys(classes).map(
  (key) => classes[key]
);
console.log("initializing roots");
for (const className of Object.keys(classes)) {
  const klass = classes[className];
  console.log("found exported class", className);
  if (klass.id) {
    console.log("checking root", klass.id);
    const object = getRoot(klass.id);

    if (object) {
      console.log(`root ${klass.id} already exists`);
    }

    if (!object) {
      console.log(`initializing root ${klass.id} with class ${className}`);
      const root = new klass();
      setRoot(klass.id, root);
    }
  }
}

commit();
