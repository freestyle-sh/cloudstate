{
  const blob = new Blob(new Uint8Array([1, 2, 3, 4]));

  setRoot("test-root", {
    value: {
      a: blob,
    },
  });
}

// END_FILE

async function main() {
  const object = getRoot("test-root");

  if (!(object.value.a instanceof Blob)) {
    throw new Error(
      `Expected object.value.a to be a Blob. Got ${object.value.a}`,
    );
  }

  object.value.a.bytes().then(async (bytes) => {
    // check types
    if (!(bytes instanceof Uint8Array)) {
      throw new Error(
        `Expected bytes to be a Uint8Array. Got ${bytes}`,
      );
    }

    const expected = await new Blob(new Uint8Array([1, 2, 3, 4])).bytes();

    console.log("BYTES", bytes, bytes.constructor);

    // check values
    if (bytes.length !== expected.length) {
      throw new Error(
        `Expected bytes to have length ${expected.length}. Got ${bytes.length}`,
      );
    }

    for (let i = 0; i < expected.length; i++) {
      if (bytes[i] !== expected[i]) {
        throw new Error(
          `Expected bytes[${i}] to be ${expected[i]}. Got ${bytes[i]}`,
        );
      }
    }
  });
}

main();
