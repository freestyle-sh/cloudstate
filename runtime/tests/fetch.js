{
  const object = {
    value: "",
  };

  const res = await fetch("http://example.com/");
  object.value = await res.text();

  setRoot("test-root", object);
  commit();
}

// END_FILE

{
  const object = getRoot("test-root");
  if (!object) {
    throw new Error("object should exist");
  }
  if (!object.value.includes("Example Domain")) {
    throw new Error("object.value should include 'Example Domain'");
  }
}
