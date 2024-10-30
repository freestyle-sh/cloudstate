{
  const blob = new Blob(new Uint8Array([1, 2, 3, 4]), { type: "text/plain" });

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

  object.value.a.arrayBuffer().then(async (buffer) => {
    // check types
    if (!(buffer instanceof ArrayBuffer)) {
      throw new Error(
        `Expected buffer to be an ArrayBuffer. Got ${buffer}`,
      );
    }

    const expected = await new Blob(new Uint8Array([1, 2, 3, 4]), {
      type: "text/plain",
    })
      .arrayBuffer();

    // check values
    if (buffer.byteLength !== expected.byteLength) {
      throw new Error(
        `Expected buffer to have length ${expected.byteLength}. Got ${buffer.byteLength}`,
      );
    }

    const bytes = new Uint8Array(buffer);
    const expectedBytes = new Uint8Array(expected);

    for (let i = 0; i < expectedBytes.length; i++) {
      if (bytes[i] !== expectedBytes[i]) {
        throw new Error(
          `Expected buffer[${i}] to be ${expectedBytes[i]}. Got ${bytes[i]}`,
        );
      }
    }
  });
}

main();
