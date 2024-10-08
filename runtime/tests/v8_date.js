{
  const base = new Date(42);
  const object = {
    date: base,
  };

  setRoot("test-root", object);
  commit();
}

// END_FILE

{
  const expected = new Date(42);
  const object = getRoot("test-root");
  if (!object) {
    throw new Error("object should exist");
  }
  if (!object.date) {
    throw new Error("object.date should exist");
  }
  if (object.date instanceof Date === false) {
    throw new Error("object.date should be a Date");
  }
  if (object.date.getTime() !== expected.getTime()) {
    throw new Error("object.date should be the same as baseDate");
  }
  if (object.date.getFullYear() !== expected.getFullYear()) {
    throw new Error("object.date should be the same as baseDate");
  }
  if (object.date.getMonth() !== expected.getMonth()) {
    throw new Error("object.date should be the same as baseDate");
  }
  if (object.date.getDate() !== expected.getDate()) {
    throw new Error("object.date should be the same as baseDate");
  }
  if (object.date.getHours() !== expected.getHours()) {
    throw new Error("object.date should be the same as baseDate");
  }
  if (object.date.getMinutes() !== expected.getMinutes()) {
    throw new Error("object.date should be the same as baseDate");
  }
  if (object.date.getSeconds() !== expected.getSeconds()) {
    throw new Error("object.date should be the same as baseDate");
  }
  if (object.date.getMilliseconds() !== expected.getMilliseconds()) {
    throw new Error("object.date should be the same as baseDate");
  }
}
