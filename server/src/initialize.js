globalThis.process = {
  env: env_string,
};

const classes = await import("./lib.js").catch((e) => {
  console.error("Error importing classes", e);
  throw e;
});

for (const className of Object.keys(classes)) {
  const klass = classes[className];
  registerCustomClass(klass);
}

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

commit();
