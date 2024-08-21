const baseDate = new Date();
{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const object = transaction.getRoot("test-root") || {
    date: baseDate,
  };

  transaction.setObject(object);
  transaction.setRoot("test-root", object);

  transaction.commit();
}
{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const object = transaction.getRoot("test-root");
  console.log("OUTPUT WITH DATE", object, typeof object.date, object.date.getFullYear());

  if (!object) throw new Error("object should exist");
  if (!object.date) throw new Error("object.date should exist");
  if (object.date instanceof Date === false) throw new Error("object.date should be a Date");
  if (object.date.getTime() !== baseDate.getTime()) throw new Error("object.date should be the same as baseDate");
  if (object.date.getFullYear() !== baseDate.getFullYear()) throw new Error("object.date should be the same as baseDate");
  if (object.date.getMonth() !== baseDate.getMonth()) throw new Error("object.date should be the same as baseDate");
  if (object.date.getDate() !== baseDate.getDate()) throw new Error("object.date should be the same as baseDate");
  if (object.date.getHours() !== baseDate.getHours()) throw new Error("object.date should be the same as baseDate");
  if (object.date.getMinutes() !== baseDate.getMinutes()) throw new Error("object.date should be the same as baseDate");
  if (object.date.getSeconds() !== baseDate.getSeconds()) throw new Error("object.date should be the same as baseDate");
  if (object.date.getMilliseconds() !== baseDate.getMilliseconds()) throw new Error("object.date should be the same as baseDate");

}
