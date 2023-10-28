import { Queue } from './utils';

const worker = new Worker(
  /* webpackChunkName: "rs-worker" */ new URL('./worker.js', import.meta.url)
);

/**
 * @type { HTMLCanvasElement }
 */
const canvas = document.querySelector('#canvas');
const bridgeCanvas = document.querySelector('#bridge');
const video = document.querySelector('#video');
const btnInitWasm = document.getElementById("btn_init_wasm");
const btn1 = document.getElementById("btn1");
const btn2 = document.getElementById("btn2");

const btn3 = document.getElementById("btn_select_file");
const btn4 = document.getElementById("btn_reset");
const btn5 = document.getElementById("btn_redraw");

const range1 = document.getElementById("low_threshold");
const range2 = document.getElementById("high_threshold");

let isReady = false;

let threshold_low = 100;
let threshold_high = 200;

const taskQueue = new Queue();

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

    initCamera();
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
    drawOriginalImage(imgUrl);
    worker.postMessage({ type: "DRAW_IMAGE", data: imgUrl });
    taskQueue.push({ type: "DRAW_IMAGE", data: imgUrl });
  });
  
  btn2.addEventListener('click', e => {
    checkReady();
    const imgUrl = `${process.env.ASSET_PATH}2.png`;
    const img = new Image();
    img.onload = () => {
      drawOriginalImage(img);
      window.createImageBitmap(img).then(imgBitmap => {
        worker.postMessage({ type: "DRAW_EDGE", data: {
          bitmap: imgBitmap,
          low: threshold_low,
          high: threshold_high
        }}, [imgBitmap]);
      });
      window.createImageBitmap(img).then(bitmapCopy => {
        taskQueue.push({ 
          type: "DRAW_EDGE",
          data: {
            bitmap: bitmapCopy,
            low: threshold_low,
            high: threshold_high
          },
          transfer: [bitmapCopy],
        })
      });
    }
    img.src = imgUrl;
  });
  
  btn3.addEventListener('click', e => {
    checkReady();
    const fileInput = document.createElement('input');
    fileInput.type = 'file';
    fileInput.accept = 'image/*';
    fileInput.onchange = loadFileAsUrl;
    fileInput.click();
  });

  btn4.addEventListener('click', e => {
    checkReady();
    const canvas = document.body.querySelector('#original');
    const ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    worker.postMessage({ type: "RESET" });
  });

  range1.value = threshold_low;
  range2.value = threshold_high;

  range1.addEventListener('change', e => {
    threshold_low = +e.target.value;
  });

  range2.addEventListener('change', e => {
    threshold_high = +e.target.value
  });

  btn5.addEventListener('click', async e => {
    const task = await taskQueue.head();
    if (!task) {
      return;
    }
    worker.postMessage({
      type: task.type,
      data: task.type === 'DRAW_EDGE' ? {
        ...task.data,
        low: threshold_low,
        high: threshold_high
      } : task.data,
    }, task.transfer);
  });
}

function drawOriginalImage(img) {
  const canvas = document.body.querySelector('#original');
  const ctx = canvas.getContext('2d');
  if (typeof img === 'string') {
    const image = new Image();
    image.onload = function() {
      ctx.canvas.width = image.width;
      ctx.canvas.height = image.height;
      ctx.drawImage(image, 0, 0);
    }
    image.src = img;
  } else if (img instanceof HTMLImageElement) {
    ctx.canvas.width = img.width;
    ctx.canvas.height = img.height;
    ctx.drawImage(img, 0, 0);
  }
}

function loadFileAsUrl(e) {
  const file = e.target.files[0];
  const reader = new FileReader();
  reader.onload = function(e) {
    const url = e.target.result;
    drawOriginalImage(url);
    const img = new Image();
    img.onload = () => {
      window.createImageBitmap(img).then(imgBitmap => {
        worker.postMessage({ type: "DRAW_EDGE", data: {
          bitmap: imgBitmap,
          low: threshold_low,
          high: threshold_high
        }}, [imgBitmap]);
        window.createImageBitmap(img).then(bitmapCopy => {
          taskQueue.push({
            type: "DRAW_EDGE",
            data: {
              bitmap: bitmapCopy,
              low: threshold_low,
              high: threshold_high
            },
            transfer: [bitmapCopy]
          })
        })
        
      })
    }
    img.src = url;
  }
  reader.readAsDataURL(file);
}

function initCamera() {
  navigator.mediaDevices.getUserMedia({ video: true })
    .then(stream => {
      video.srcObject = stream;
      const videoTrack = stream.getVideoTracks()[0];
      const { width, height } = videoTrack.getSettings();
      function onFrame() {
        // const videoFrame = getVideoFrameData(video, width, height);
        // worker.postMessage({
        //   type: 'VIDEO_FRAME',
        //   data: videoFrame,
        // }, [videoFrame]);
        getImgbitmapFromVideo(video, width, height).then(bitmap => {
          worker.postMessage({
            type: 'VIDEO_FRAME',
            data: bitmap,
          }, [bitmap]);
        });
        requestAnimationFrame(onFrame);
      }

      requestAnimationFrame(onFrame);
    })
    .catch(err => {
      console.error(err);
    });
}

/**
 * @deprecated not transferable
 */
function getVideoFrameData(video, w, h) {
  /**@type {CanvasRenderingContext2D} */
  const ctx = bridgeCanvas.getContext('2d');
  canvas.width = w;
  canvas.height = h;
  ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
  return ctx.getImageData(0, 0, canvas.width, canvas.height);
}

function getImgbitmapFromVideo(video, w, h) {
  const ctx = bridgeCanvas.getContext('2d');
  canvas.width = w;
  canvas.height = h;
  ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
  return window.createImageBitmap(canvas);
}