import * as wasm from "./pkg/rs_js.js";

self.onmessage = async ({data: { type, data }}) => {
  switch(type) {
    case "INIT":
      /**
       * When we receive the bytes as an `ArrayBuffer` we can use that to
       * synchronously initialize the module as opposed to asynchronously
       * via the default export. The synchronous method internally uses
       * `new WebAssembly.Module()` and `new WebAssembly.Instance()`.
       */
      wasm.initSync(data);
      break;
    case "RUN":
      const canvas = data;
      const ctx = canvas.getContext('2d');
      if(!ctx) {
        postMessage({error: 'unsupported browser'});
        return;
      }
      const imgBlob = await fetch('https://upload.wikimedia.org/wikipedia/commons/5/55/John_William_Waterhouse_A_Mermaid.jpg')
        .then(r => r.blob());
      const img = await createImageBitmap(imgBlob);
      wasm.drawImageData(ctx, img);
      break;
    default:
      break;
  }

};

/**
 * Once the Web Worker was spawned we ask the main thread to fetch the bytes
 * for the WebAssembly module. Once fetched it will send the bytes back via
 * a `postMessage` (see above).
 */
self.postMessage({ type: "FETCH_WASM" });