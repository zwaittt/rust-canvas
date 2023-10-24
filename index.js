const worker = new Worker(
  /* webpackChunkName: "rs-worker" */ new URL('./worker.js', import.meta.url)
);

/**
 * @type { HTMLCanvasElement }
 */
const canvas = document.querySelector('#canvas');
const btnInitWasm = document.getElementById("btn_init_wasm");
const btn1 = document.getElementById("btn1");
const btn2 = document.getElementById("btn2");

let isReady = false;

function checkReady() {
  if (!isReady) {
    throw alert('Please click `init wasm` firstly.');
  }
}

worker.addEventListener('message', e => {
  if (e.data === 'worker_created') {
    console.log('========== worker successfully initialized ==========');
    initWorkerImpl();
    initInteractions();
  }
});

function initWorkerImpl() {
  worker.onmessage = ({ data }) => {
    if (data.error) {
      console.error(data);
      return;
    }
    const { type } = data;
  
    switch (type) {
      case 'WASM_INIT_DONE':
      console.log('========== wasm successfully initialized ==========');
      const offscreen = canvas.transferControlToOffscreen();
      worker.postMessage({ type: "CANVAS", data: offscreen }, [offscreen]);
      isReady = true;
      btnInitWasm.removeEventListener('click', initWasm);
      break;
  
      default: {
        break;
      }
    }
  };
}

function initWasm() {
  fetch(`${process.env.ASSET_PATH}rs_js_bg.wasm`)
      .then((response) => response.arrayBuffer())
      .then((bytes) => {
        worker.postMessage({ type: "INIT", data: bytes }, [bytes]);
      });
}

function initInteractions() {
  btnInitWasm.addEventListener('click', initWasm);
  
  btn1.addEventListener('click', e => {
    checkReady();
    const imgUrl = 'https://upload.wikimedia.org/wikipedia/commons/5/55/John_William_Waterhouse_A_Mermaid.jpg';
    worker.postMessage({ type: "DRAW_IMAGE", data: imgUrl });
  });
  
  btn2.addEventListener('click', e => {
    checkReady();
    const imgUrl = `${process.env.ASSET_PATH}2.png`;
    const img = new Image();
    img.onload = () => {
      window.createImageBitmap(img).then(imgBitmap => {
        worker.postMessage({ type: "DRAW_EDGE", data: imgBitmap }, [imgBitmap]);
      })
      
    }
    img.src = imgUrl;
  });
}