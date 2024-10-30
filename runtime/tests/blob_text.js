{
  const blob = new Blob(["Hello, world!"], { type: "text/plain" });

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

  object.value.a.text().then(async (text) => {
    // check types
    if (typeof text !== "string") {
      throw new Error(
        `Expected text to be a string. Got ${text}`,
      );
    }

    const expected = await new Blob(["Hello, world!"], { type: "text/plain" })
      .text();

    if (text !== expected) {
      throw new Error(
        `Expected text to be ${expected}. Got ${text}`,
      );
    }
  });
}
