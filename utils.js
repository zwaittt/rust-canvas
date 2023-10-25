export class Queue {
  
  _queue = [];
  max = 20;
  
  push(item) {
    if (this._queue.length >= this.max) {
      this._queue.shift();
    }
    this._queue.push(item);
  }
  pop() {
    return this._queue.shift();
  }
  get length() {
    return this._queue.length;
  }
  clear() {
    this._queue = [];
  }

  async head() {
    if (!this.length) {
      return;
    }
    let _head = this._queue.pop();

    if (_head.transfer && _head.transfer.length && _head.transfer[0] instanceof ImageBitmap) {
      const bitmap = await copyImageBitmap(_head.transfer[0]);
      this._queue.push({
        ..._head,
        data: {
          ..._head.data,
          bitmap,
        },
        transfer: [bitmap],
      })
    } else {
      this._queue.push({
        ..._head,
      })
    }
    return _head;
  }
}

async function copyImageBitmap(imageBitmap) {
  const canvas = document.createElement('canvas');
  canvas.width = imageBitmap.width;
  canvas.height = imageBitmap.height;

  const ctx = canvas.getContext('2d');
  ctx.drawImage(imageBitmap, 0, 0);

  const copiedImageBitmap = await createImageBitmap(canvas);
  return copiedImageBitmap;
}