import * as classes from "./lib.js";
globalThis.cloudstate.customClasses = Object.keys(classes).map(
  (key) => classes[key]
);

let bytes = new Uint8Array([]);

const object = getRoot(test);
try {
  const out = object["fetch"]();

  if (out instanceof Promise) {
    out = await globalThis.result;
  }

  if (out instanceof Response) {
    const body = await out.bytes();
    const headers = [...out.headers.entries()];

    // uint8array to array
    let bytes = Array.from(body);

    globalThis.result = { result: { bytes, headers } };
  }
} catch (e) {
  globalThis.result = { error: { message: e.message, stack: e.stack } };
}
