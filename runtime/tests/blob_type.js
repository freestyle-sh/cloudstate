{
  const blob = new Blob(["hello world"], { type: "text/plain" });

  setRoot("test-root", {
    value: {
      a: blob,
    },
  });
}

// END_FILE

{
  const object = getRoot("test-root");
  console.log("OBJECT b4");
  console.log("OBJECT", object);

  if (!(object.value.a instanceof Blob)) {
    throw new Error(
      `Expected object.value.a to be a Blob. Got ${object.value.a}`,
    );
  }

  if (object.value.a.type !== "text/plain") {
    throw new Error(
      `Expected object.value.a.type to be 'text/plain'. Got ${object.value.a.type}`,
    );
  }
}
