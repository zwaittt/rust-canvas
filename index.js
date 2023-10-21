const worker = new Worker(
  /* webpackChunkName: "rs-worker" */ new URL('./worker.js', import.meta.url)
);

/**
 * @type { HTMLCanvasElement }
 */
const canvas = document.querySelector('#canvas');

worker.onmessage = ({ data }) => {
  if (data.error) {
    console.error(data);
    return;
  }
  const { type } = data;

  switch (type) {
    case "FETCH_WASM": {
      fetch("/rs_js_bg.wasm")
        .then((response) => response.arrayBuffer())
        .then((bytes) => {
          worker.postMessage({ type: "INIT", data: bytes }, [bytes]);
        });
      break;
    }

    default: {
      break;
    }
  }
};

const btn = document.getElementById("btn");

btn.addEventListener('click', e => {
  const offscreen = canvas.transferControlToOffscreen();
  worker.postMessage({ type: "RUN", data: offscreen }, [offscreen]);
});