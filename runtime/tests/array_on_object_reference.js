{
  console.log("START_FILE");

  const object = {};

  setRoot("test-root", object);

  commit();
}

// END_FILE

{
  const object = getRoot("test-root");

  object.value = [];

  commit();
}
