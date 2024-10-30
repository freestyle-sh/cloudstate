{
  let testBlob = new Blob(new Uint8Array([1, 2, 3, 4]), { type: "text/plain" });
  setRoot("test-root", {
    value: {
      a: testBlob,
    },
  });
}

// END_FILE

{
  let testBlob = new Blob(new Uint8Array([1, 2, 3, 4]), { type: "text/plain" });

  let object = getRoot("test-root");
  let blob = object.value.a;

  if (!(blob instanceof Blob)) {
    throw new Error(`Expected object.value.a to be a Blob. Got ${blob}`);
  }

  blob.slice(null, 2).arrayBuffer().then(async (buffer) => {
    let expected = await testBlob.slice(null, 2).arrayBuffer();

    if (buffer.byteLength !== expected.byteLength) {
      throw new Error(
        `Expected buffer to have length ${expected.byteLength}. Got ${buffer.byteLength}`,
      );
    }

    let bytes = new Uint8Array(buffer);
    let expectedBytes = new Uint8Array(expected);

    for (let i = 0; i < expectedBytes.length; i++) {
      if (bytes[i] !== expectedBytes[i]) {
        throw new Error(
          `Expected buffer[${i}] to be ${expectedBytes[i]}. Got ${bytes[i]}`,
        );
      }
    }
  });
}
