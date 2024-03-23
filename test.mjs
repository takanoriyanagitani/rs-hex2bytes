import { readFile } from "node:fs/promises";

(() => {
  return Promise.resolve("rs_hex2bytes.wasm")
    .then(readFile)
    .then((bytes) => WebAssembly.instantiate(bytes))
    .then((pair) => {
      const {
        module,
        instance,
      } = pair || {};
      const {
        memory,

        input_resize,
        input_ptr,

        output_reset,
        output_ptr,

        hex2bytes_std,
        hex2bytes_std_chunk8,
        hex2bytes_chunk8simd,
      } = instance?.exports || {};

      const hex365le = "0000124200001242";
      const ilen = hex365le.length;

      const icap = input_resize(ilen);
      console.info({ icap });

      const ocap = output_reset(ilen >> 1);
      console.info({ ocap });

      const iview = new Uint8Array(memory?.buffer, input_ptr(), ilen);
      const enc = new TextEncoder();
      enc.encodeInto(hex365le, iview);

      //const len = hex2bytes_std()
      const len = hex2bytes_chunk8simd();
      const oview = new Uint8Array(memory?.buffer, output_ptr(), len);
      const obuf = Buffer.from(oview);
      return obuf.readFloatLE(0);
    })
    .then(console.info)
    .catch(console.warn);
})();
