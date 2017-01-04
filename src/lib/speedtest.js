
export default function SpeedTest (opts) {
  this.init(opts)
}

function SpeedTestSocket (opts) {
  this.init(opts)
}

function fnow () {
  return window.performance ? window.performance.now() : Date.now()
}

let uploadBuffers = {
  1000: new ArrayBuffer(1000),
  10000: new ArrayBuffer(10000),
  100000: new ArrayBuffer(100000)
}
let uploadSize
let uploadBuf

SpeedTest.prototype = {
  constructor: SpeedTest,

  init (opts) {
    this.opts = opts
    this.sts = []
    this.rates = []
  },

  connect (opts) {
    Object.assign(this.opts, opts)
    this.lastRate = 0
    uploadSize = 10000
    uploadBuf = uploadBuffers[uploadSize]
    for (let i = 0; i < this.opts.numParallel; i++) {
      this.sts[i] = new SpeedTestSocket(this.opts)
    }
    let p = []
    for (let i in this.sts) {
      console.log('connecting', i)
      p.push(this.sts[i].connect())
    }
    return Promise.all(p)
  },

  runTest () {
    console.log('runTest called')
    let p = []
    for (let i in this.sts) {
      console.log('starting', i)
      p.push(this.sts[i].start())
    }
    return Promise.all(p)
  },

  average (rate) {
    this.rates.push(rate)
    let start = 0
    let len = this.rates.length
    if (len < 8) {
      len += 3
    }
    if (len > 10) {
      start = (len * 0.3) | 0
      len = (len * 0.6) | 0
    }
    let tot = 0
    for (let i = 0; i < len; i++) {
      tot += this.rates[start + i] || 0
    }
    return tot / len
  },

  lowpass (val, oldval) {
    let maxd = ((val + oldval) / 2) * 0.5
    if (Math.abs(oldval - val) > maxd) {
      val = val > oldval ? oldval + maxd : oldval - maxd
    }
    return val
  },

  poll () {
    // calculate current statistics.
    let r = {
      curBytes: 0,
      curRate: 0,
      curSecs: 0
    }
    for (let i = 0; i < this.sts.length; i++) {
      r.curBytes += this.sts[i].curBytes
      r.curRate += this.sts[i].curRate
      if (this.sts[i].curSecs > r.curSecs) {
        r.curSecs = this.sts[i].curSecs
      }
    }

    r.curRate = this.lowpass(r.curRate, this.lastRate)
    r.avgRate = this.average(r.curRate)
    this.lastRate = r.curRate

    // see if we need to use larger buffers.
    // console.log('XXX', this, this.opts, r, uploadSize)
    if (this.opts.isUpload && uploadSize < 100000) {
      let newSize = uploadSize
      let n = r.curRate / this.opts.numParallel
      if (n > 0.05) {
        newSize = 10000
      }
      if (n > 0.5) {
        newSize = 100000
      }
      // console.log('upload adjust n is', n)
      if (uploadSize !== newSize) {
        // console.log('adjust uploadsize to', newSize)
        uploadSize = newSize
        uploadBuf = uploadBuffers[uploadSize]
      }
    }

    return r
  },

  stop () {
    for (let i = 0; i < this.sts.length; i++) {
      this.sts[i].stop()
    }
    this.sts = []
  }
}

SpeedTestSocket.prototype = {
  constructor: SpeedTestSocket,

  init (opts) {
    this.opts = opts
  },

  clear () {
    Object.assign(this, {
      // configurable
      maxTestMS: 8000,
      // runtime data
      curBytes: 0,
      curRate: 0,
      curSecs: 0,
      maxRate: 0,
      maxBytes: 0,
      maxDeltaMS: 0,
      lastMS: 0,
      lastBytes: 0,
      startMS: 0,
      ws: null
    }, this.opts)
  },

  connect () {
    this.clear()
    let ret = new Promise((resolve, reject) => {
      this.resolve = resolve
      this.reject = reject
    })
    try {
      var ws = new window.WebSocket(this.url)
    } catch (e) {
      console.log('socket exception', e)
      this.reject(e)
    }
    console.log('new websocket for', this.url)
    ws.binaryType = 'arraybuffer'
    ws.onclose = () => {
      console.log('socket unexpectedly closed')
      this.reject()
    }
    ws.onerror = (e) => {
      console.log('websocket error', e)
      this.ws.close()
      this.reject(e)
    }
    ws.onmessage = (e) => {
      this.updateProgress(e)
      if (this.lastMS - this.startMS >= this.maxTestMS) {
        this.stop()
      }
    }
    ws.onopen = () => {
      console.log('websocket connected')
      this.resolve()
    }
    this.ws = ws
    return ret
  },

  pushData () {
    if (!this.sending) {
      return
    }
    if (this.ws.bufferedAmount === 0) {
      this.ws.send(uploadBuf)
    }
    window.setTimeout(this.pushData.bind(this), 0)
  },

  start () {
    let ret = new Promise((resolve, reject) => {
      this.resolve = resolve
      this.reject = reject
    })
    this.ws.onclose = () => {
      console.log('socket closed')
      this.resolve()
    }
    if (this.isUpload) {
      this.sending = true
      this.pushData()
    } else {
      this.ws.send(JSON.stringify({
        download: 'start',
        messagesize: 100000
      }))
    }
    return ret
  },

  stop () {
    console.log('stop called')
    this.sending = false
    this.ws.close()
  },

  updateProgress (e) {
    let now, size
    if (this.isUpload) {
      // console.log('upload, got JSON', e.data)
      let r = JSON.parse(e.data)
      now = r.timestamp
      size = r.messagesize
    } else {
      now = fnow()
      if (e.data.size) {
        size = e.data.size
      } else if (e.data.length) {
        size = e.data.length
      } else if (e.data.byteLength) {
        size = e.data.byteLength
      }
    }
    this.curBytes += size

    if (this.lastMS === 0) {
      this.startMS = now
      this.lastMS = now
      return
    }

    if (this.lastMS > now - 100) {
      return
    }
    let deltaMS = now - this.lastMS
    this.lastMS = now

    let deltaBytes = this.curBytes - this.lastBytes
    this.lastBytes = this.curBytes

    this.curSecs = (now - this.startMS) / 1000

    let rate = 0
    if (deltaMS > 0) {
      rate = deltaBytes / (deltaMS / 1000)
    }
    if (rate > this.maxRate) {
      this.maxRate = rate
      this.maxBytes = deltaBytes
      this.maxDelta = deltaMS
    }
    this.curRate = rate / 1000000
  }
}
