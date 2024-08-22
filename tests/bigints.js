const basebigInt = BigInt("1234567890123456789012345678901234567890");
{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();
  const object = transaction.getRoot("test-root") || {
    big: basebigInt,
  };
  transaction.setObject(object);
  transaction.setRoot("test-root", object);

  transaction.commit();
}
{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const object = transaction.getRoot("test-root");

  if (!object) throw new Error("object should exist");
  if (!object.big) throw new Error("object.big should exist");
  if (typeof object.big !== "bigint") throw new Error("object.big should be a bigint");
}
