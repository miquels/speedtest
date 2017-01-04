<template>
<div class="radial-gauge">
  <div class="radial-gauge__container">
    <canvas class="radial-gauge__bg"></canvas>
    <canvas class="radial-gauge__fg"></canvas>
  </div>
</div>
</template>

<script>
export default {
  name: 'radial-gauge',
  data: () => ({
    fontWidth: 0,
    fontHeight: 0,
    xpos: 0,
    ypos: 0
  }),
  props: {
    minsize: { type: Number, default: 0 },
    size: { type: Number, default: 0 },
    units: { type: String, default: 'Mbit/s' },
    majorTicks: {
      type: [ Array, Object ],
      default: () => ([ 0, 1, 5, 20, 50, 100, 200, 500, 1000 ])
    },
    minorTicks: {
      type: [ Array, Object ],
      default: () => ([
        [ 0, 1, 0.5 ], [ 1, 5, 1 ], [ 5, 20, 5 ], [ 20, 50, 10 ],
        [ 50, 100, 25 ], [ 100, 200, 50 ], [ 200, 1000, 100 ]
      ])
    },
    transformValue: {
      default: () => (val) => (Math.log(val + 1) * 100 / Math.log(1001))
    },
    value: { type: Number, default: 0 },
    maxValue: { type: Number, default: 1000 },
    precision: { type: Number, default: 1 }
  },
  watch: {
    value (val) {
      this.draw(val)
    }
  },
  mounted () {
    this.canvas = this.$el.getElementsByTagName('CANVAS')
    this.$on('resize', () => {
      this.resize()
    })
    window.Gauge = this
    this.resize()
  },
  methods: {
    resize () {
      let size = this.size
      if (size === 0) {
        let w = this.$el.clientWidth
        let h = this.$el.clientHeight
        size = Math.min(w, h)
      }
      if (this.minsize > 0 && size < this.minsize) {
        size = this.minsize
      }
      this.init(size)
      this.drawBackground()
      this.draw(this.value < 0 ? 0 : this.value)
    },
    init (size) {
      this.ctx0 = this.canvas[0].getContext('2d')
      this.ctx1 = this.canvas[1].getContext('2d')
      let dpr = window.devicePixelRatio || 1
      this.csize = size * (dpr < 1.5 ? 2 : dpr)
      // set canvas width and height here, instead of via
      // v-bind attributes on the element, since otherwise
      // vue will reset the element.
      for (let c = 0; c < 2; c++) {
        this.canvas[c].style.width = size + 'px'
        this.canvas[c].style.height = size + 'px'
        this.canvas[c].width = this.csize
        this.canvas[c].height = this.csize
      }
      this.xpos = this.csize / 2
      this.ypos = this.csize / 2
      this.outer = this.csize / 2 - 3
      this.radius = Math.round(0.95 * this.outer)

      this.fontsize0 = Math.round(this.csize / 18)
      this.fontsize1 = Math.round(this.csize / 16)
      this.ctx0.font = this.fontsize0 + 'px sans-serif'
      this.ctx1.font = this.fontsize1 + 'px "Lucida Console",Monaco,monospace'
      this.fontHeight = this.fontsize1
      this.fontWidth = this.ctx1.measureText('0').width
      this.lastPct = 0
    },
    pos (angle, radius) {
      let phi = (angle / 180) * Math.PI
      let x = this.xpos - radius * Math.sin(phi)
      let y = this.ypos + radius * Math.cos(phi)
      return [x, y]
    },
    drawTick (ctx, label, angle, len) {
      ctx.save()
      ctx.lineWidth = Math.round(0.5 + this.csize / 300)
      ctx.beginPath()
      ctx.moveTo(...this.pos(angle, this.radius))
      ctx.lineTo(...this.pos(angle, this.radius - len))
      ctx.stroke()
      label = '' + label
      let [x, y] = this.pos(angle, this.radius - len - 0.15 * this.radius)
      ctx.fillStyle = '#000'
      ctx.textAlign = 'center'
      ctx.fillText(label, x, y)
      ctx.restore()
    },
    drawNeedle (ctx, angle, len) {
      ctx.save()
      ctx.lineWidth = Math.round(0.5 + this.csize / 300)
      ctx.beginPath()
      ctx.strokeStyle = '#666666'
      ctx.fillStyle = '#666666'
      ctx.moveTo(this.xpos, this.ypos)
      ctx.lineTo(...this.pos(angle + 90, this.radius / 25))
      ctx.lineTo(...this.pos(angle, len))
      ctx.closePath()
      ctx.fill()
      ctx.stroke()
      ctx.beginPath()
      ctx.moveTo(this.xpos, this.ypos)
      ctx.lineTo(...this.pos(angle - 90, this.radius / 25))
      ctx.lineTo(...this.pos(angle, len))
      ctx.closePath()
      ctx.fillStyle = '#aaaaaa'
      ctx.fill()
      ctx.stroke()
      ctx.restore()
    },
    drawKnob (ctx, size) {
      ctx.save()
      ctx.beginPath()
      ctx.strokeStyle = '#888888'
      ctx.fillStyle = '#cccccc'
      ctx.arc(this.xpos, this.ypos, size, 0, 2 * Math.PI)
      ctx.fill()
      ctx.stroke()
      ctx.restore()
    },
    drawFace (ctx) {
      ctx.save()
      ctx.beginPath()
      ctx.lineWidth = Math.round(0.5 + this.csize / 300)
      ctx.arc(this.xpos, this.ypos, this.radius, 0.75 * Math.PI, 2.25 * Math.PI)
      ctx.stroke()
      ctx.beginPath()
      ctx.lineWidth = Math.round(0.5 + this.csize / 350)
      ctx.strokeStyle = '#dddddd'
      ctx.arc(this.xpos, this.ypos, this.outer, 0, 2 * Math.PI)
      ctx.stroke()
      ctx.restore()
    },
    drawLegend (ctx, text) {
      ctx.save()
      ctx.fillStyle = '#000'
      ctx.textAlign = 'center'
      ctx.fillText(text, this.xpos, this.ypos + this.radius / 2.5)
      ctx.restore()
    },
    drawTicks (ctx) {
      ctx.save()
      for (let i in this.majorTicks) {
        let v = this.transformValue(this.majorTicks[i])
        this.drawTick(ctx, this.majorTicks[i], 45 + 2.7 * v, 0.15 * this.radius)
      }
      for (let i in this.minorTicks) {
        let x = this.minorTicks[i]
        for (let a = x[0]; a < x[1]; a += x[2]) {
          if (this.majorTicks.indexOf(a) < 0) {
            let v = this.transformValue(a)
            this.drawTick(ctx, '', 45 + 2.7 * v, 0.05 * this.radius)
          }
        }
      }
      ctx.restore()
    },
    roundedBox (ctx, x, y, width, height, radius) {
      x -= width / 2
      y -= height / 2
      ctx.beginPath()
      ctx.moveTo(x + radius, y)
      ctx.lineTo(x + width - radius, y)
      // right upper corner
      ctx.arcTo(x + width, y, x + width, y + radius, radius)
      ctx.lineTo(x + width, y + height - radius)
      // right lower corder
      ctx.arcTo(x + width, y + height, x + width - radius, y + height, radius)
      ctx.lineTo(x + radius, y + height)
      // left lower corner
      ctx.arcTo(x, y + height, x, y + height - radius, radius)
      ctx.lineTo(x, y + radius)
      // left upper corner
      ctx.arcTo(x, y, x + radius, y, radius)
      ctx.fill()
      ctx.stroke()
    },
    drawValueBox (ctx) {
      ctx.save()
      let width = 8 * this.fontWidth
      let height = 1.8 * this.fontHeight
      let ypos = this.ypos + 0.7 * this.radius
      ctx.strokeStyle = '#808080'
      ctx.fillStyle = '#808080'
      this.roundedBox(ctx, this.xpos, ypos, width * 1.1, height * 1.1, height * 1.1 / 3)
      ctx.strokeStyle = '#000'
      ctx.fillStyle = '#191919'
      this.roundedBox(ctx, this.xpos, ypos, width, height, height / 3)

      ctx.restore()
    },
    drawValue (ctx, value) {
      ctx.save()
      let ypos = this.ypos + 0.7 * this.radius
      ctx.fillStyle = '#ffc300'
      ctx.textBaseline = 'middle'
      ctx.textAlign = 'center'
      let text = '' + value.toFixed(this.precision)
      ctx.fillText(text, this.xpos, ypos)
      ctx.restore()
    },
    drawBackground () {
      let ctx = this.ctx0
      ctx.clearRect(0, 0, this.canvas[0].width, this.canvas[0].height)
      this.drawFace(ctx)
      this.drawTicks(ctx)
      this.drawLegend(ctx, this.units)
      this.drawValueBox(ctx)
    },
    draw (value) {
      let pct
      if (value < 0) {
        if (this.lastPct === 0) {
          return
        }
        value = 0
        pct = (this.lastPct > 3) ? this.lastPct - 3 : 0
        window.requestAnimationFrame(() => this.draw(-1))
      } else {
        pct = this.transformValue(value)
      }
      this.lastPct = pct
      let angle = 45 + 270 * (pct / 100)
      let ctx = this.ctx1
      ctx.clearRect(0, 0, this.canvas[1].width, this.canvas[1].height)
      this.drawValue(ctx, value)
      this.drawNeedle(ctx, angle, 0.9 * this.radius)
      this.drawNeedle(ctx, angle + 180, 0.35 * this.radius)
      this.drawKnob(ctx, this.radius / 10)
    }
  }
}
</script>

<style scoped>
.radial-gauge {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0px;
  overflow: hidden;
}
.radial-gauge__container {
  position: relative;
  display: inline-block;
  margin: 0px;
  padding: 0px;
}
.radial-gauge__bg {
  position: relative;
  margin: 0px;
  padding: 0px;
}
.radial-gauge__fg {
  position: absolute;
  left: 0px;
  top: 0px;
  margin: 0px;
  padding: 0px;
}
</style>
