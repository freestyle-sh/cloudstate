const blob = new Blob();

blob["text"] = async () => {
  return Deno.core.ops.op_cloudstate_blob_get_text(value.blobId);
};

blob["arrayBuffer"] = async () => {
  /* get_data now returns Array Buffer  */
  const buffer = Deno.core.ops.op_cloudstate_blob_get_array_buffer(
    value.blobId,
  );
  return buffer;
};

blob["slice"] = (start, end, type) => {
  if (start < 0 || end < 0) {
    throw new Error("start and end must be positive");
  }
  let arrBuffer = Deno.core.ops.op_cloudstate_blob_slice(
    value.blobId,
    start,
    end,
  );
  return new Blob([arrBuffer], { type: type });
};

blob["bytes"] = async () => {
  const blob = Deno.core.ops.op_cloudstate_blob_get_uint8array(
    value.blobId,
  );
  return blob;
};

Object.defineProperty(blob, "size", {
  get: () => {
    return Deno.core.ops.op_cloudstate_blob_get_size(value.blobId);
  },
});

Object.defineProperty(blob, "type", {
  get: () => {
    return Deno.core.ops.op_cloudstate_blob_get_type(value.blobId);
  },
});

Object.defineProperty(object, key, {
  value: blob,
});

objectIds.set(blob, value.blobId);
objects.set(value.blobId, blob);
