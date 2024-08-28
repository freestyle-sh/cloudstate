{
  const object = {
    map: new Map([
      [
        "a",
        {
          count: 0,
        },
      ],
    ]),
  };

  setRoot("test-root", object);
}

commit();

{
  const object = getRoot("test-root");

  if (!object) throw new Error("object should exist");
  if (!object.map) throw new Error("object.map should exist");
  if (object.map.size !== 1) throw new Error("object.map should have size 1");

  const a = object.map.get("a");
  if (!a) throw new Error("object.map.get('a') should exist");

  if (a.count !== 0)
    throw new Error(
      `object.map.get('a').count should be 0, got ${a.count}, ${JSON.stringify(
        a
      )}`
    );
}
