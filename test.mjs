import { readFile } from "node:fs/promises"

(() => {
    return Promise.resolve("rs_hex2bytes.wasm")
    .then(readFile)
    .then(bytes => WebAssembly.instantiate(bytes))
    .then(pair => {
        const {
            module,
            instance,
        } = pair || {}
        const {
            memory,

            input_resize,
            input_ptr,

            output_reset,
            output_ptr,

            hex2bytes_std,
        } = instance?.exports || {}

        const hex365le = "00001242"
        const ilen = 8

        const icap = input_resize(ilen)
        console.info({icap})

        const ocap = output_reset(ilen >> 1)
        console.info({ocap})

        const iview = new Uint8Array(memory?.buffer, input_ptr(), ilen)
        const enc = new TextEncoder()
        enc.encodeInto(hex365le, iview)

        const len = hex2bytes_std()
        const oview = new Uint8Array(memory?.buffer, output_ptr(), len)
        const obuf = Buffer.from(oview)
        return obuf.readFloatLE(0)
    })
    .then(console.info)
    .catch(console.warn)
})()
