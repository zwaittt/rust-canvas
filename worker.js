import * as wasm from "./pkg/rs_js.js";
/**
 * @type {OffscreenCanvas}
 */
let canvas;
/**
 * @type {OffscreenCanvasRenderingContext2D}
 */
let ctx;
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
      self.postMessage({type: 'WASM_INIT_DONE'});
      break;
    case "CANVAS":
      canvas = data;
      ctx = canvas.getContext('2d');
      if(!ctx) {
        postMessage({error: 'unsupported browser'});
        return;
      }
      break;
    case "DRAW_IMAGE":
      const url = data;
      const imgBlob = await fetch(url)
        .then(r => r.blob());
      const img = await createImageBitmap(imgBlob);
      wasm.drawImageData(ctx, img);
      break;
    case "DRAW_EDGE":
      if(!ctx) {
        console.error('canvas not ready');
        return;
      }
      /**
       * @type {ImageBitmap}
       */
      const imgBitmap = data;
      wasm.drawImageEdge(imgBitmap).then(_bitmap => {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        canvas.width = imgBitmap.width;
        canvas.height = imgBitmap.height;
        ctx.drawImage(_bitmap, 0, 0);
      }).catch(e => {
        console.error(e);
      })
    default:
      break;
  }

};

self.postMessage('worker_created');