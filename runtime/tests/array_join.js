const baseArray = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const joinedArray = baseArray.join(" ");
{
  const object = {
    value: [...baseArray],
  };

  setRoot("test-root", object);
  commit();
}

{
  const root = getRoot("test-root");

  if (!root) throw new Error("root should exist");

  if (!root.value) throw new Error("root.value should exist");

  if (root.value.join(" ") !== joinedArray)
    throw new Error(`root.value should be ${joinedArray}`);

  const joined_by_number = root.value.join(1);
  if (joined_by_number !== baseArray.join(1))
    throw new Error(
      `root.value should be ${baseArray.join(1)}, got ${joined_by}`
    );
}
