{
  setRoot("test-root", {
    value: {
      a: new Blob(["hello"], {
        type: "text/plain",
      }),
    },
  });
}

// END_FILE

{
  const object = getRoot("test-root");

  if (!(object.value.a instanceof Blob)) {
    throw new Error(
      `Expected object.value.a to be a Blob. Got ${object.value.a}`,
    );
  }

  if (object.value.a.size !== 5) {
    throw new Error(
      `Expected object.value.a.size to be 5. Got ${object.value.a.size}`,
    );
  }
}
