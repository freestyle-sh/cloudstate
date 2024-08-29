const baseDate = new Date();

{
  const object = {
    date: baseDateNow,
  };

  setRoot("test-root", object);
  commit();
}

{
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
  if (object.date.getTime() !== baseDateNow.getTime()) {
    throw new Error("object.date should be the same as baseDate");
  }
  if (object.date.getFullYear() !== baseDateNow.getFullYear()) {
    throw new Error("object.date should be the same as baseDate");
  }
  if (object.date.getMonth() !== baseDateNow.getMonth()) {
    throw new Error("object.date should be the same as baseDate");
  }
  if (object.date.getDate() !== baseDateNow.getDate()) {
    throw new Error("object.date should be the same as baseDate");
  }
  if (object.date.getHours() !== baseDateNow.getHours()) {
    throw new Error("object.date should be the same as baseDate");
  }
  if (object.date.getMinutes() !== baseDateNow.getMinutes()) {
    throw new Error("object.date should be the same as baseDate");
  }
  if (object.date.getSeconds() !== baseDateNow.getSeconds()) {
    throw new Error("object.date should be the same as baseDate");
  }
  if (object.date.getMilliseconds() !== baseDateNow.getMilliseconds()) {
    throw new Error("object.date should be the same as baseDate");
  }
}
