import { readFile } from "node:fs/promises"
import { randomBytes } from "node:crypto"

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

        const inputSize = 1048576
        const inputBuf = randomBytes(inputSize)
        const inputHex = inputBuf.toString("hex")

        const lpcnt = 16

        const enc = new TextEncoder()

        const started = Date.now()
        let tot_bytes = 0
        for(let i=0; i<lpcnt; i++){
            const ilen = inputHex.length

            const icap = input_resize(ilen)
            const ocap = output_reset(ilen >> 1)

            const iview = new Uint8Array(memory?.buffer, input_ptr(), ilen)
            enc.encodeInto(inputHex, iview)

            const len = hex2bytes_std()
            tot_bytes += len
        }
        const elapsed = Date.now() - started
        const bytes_per_ms = tot_bytes / elapsed

        return {elapsed, tot_bytes, bytes_per_ms}
    })
    .then(console.info)
    .catch(console.warn)
})()
