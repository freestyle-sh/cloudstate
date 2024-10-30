{
  let blob = new Blob(new Uint8Array([1, 2, 3, 4]), { type: "text/plain" });
  setRoot("test-root", {
    value: {
      a: blob,
    },
  });
}

{
  let testBlob = new Blob(new Uint8Array([1, 2, 3, 4]), { type: "text/plain" });
  let object = getRoot("test-root");
  let blob = object.value.a;
  if (!(blob instanceof Blob)) {
    throw new Error(`Expected object.value.a to be a Blob. Got ${blob}`);
  }

  if (blob.slice(1, 2, "custom/type").type !== "custom/type") {
    throw new Error(
      `Expected blob.slice(1,2, "custom/type").type to be "custom/type". Got ${
        blob.slice(1, 2, "custom/type").type
      }`,
    );
  }
}
