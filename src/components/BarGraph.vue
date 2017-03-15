<template>
<div class="bargraph">
  <div class="bargraph__canvas">
    <canvas :width="canvasWidth" :height="canvasHeight"
            :style="{width:graphWidth + 'px',height:graphHeight + 'px'}">
    </canvas>
  </div>
  <div class="bargraph__label">{{ label }}</div>
</div>
</template>

<script>
import ResizeMixin from './ResizeMixin'
export default {
  name: 'bargraph',

  mixins: [
    ResizeMixin
  ],

  data: () => ({
    scale: 2,
    graphWidth: 0,
    graphHeight: 0
  }),

  props: {
    bars: {},
    label: {},
    barWidth: { default: 0 }
  },

  watch: {
    bars () {
      this.draw(true)
    }
  },

  computed: {
    canvasWidth () { return this.graphWidth * this.scale },
    canvasHeight () { return this.graphHeight * this.scale }
  },

  created () {
    this.scale = window.devicePixelRatio || 2
  },

  mounted () {
    this.canvas = this.$el.getElementsByTagName('CANVAS')[0]
    this.canvasDiv = this.$el.getElementsByClassName('bargraph__canvas')[0]
    this.$on('resize', this.resize)
    this.resize()
  },

  methods: {
    resize () {
      this.graphWidth = this.canvasDiv.clientWidth
      this.graphHeight = this.canvasDiv.clientHeight
      this.draw(false)
    },

    draw (partial) {
      let ctx = this.canvasContext
      if (!ctx) {
        ctx = this.canvas.getContext('2d')
        this.canvasContext = ctx
      }
      if (!partial || this.bars.length === 0) {
        ctx.clearRect(0, 0, this.canvasWidth, this.canvasHeight)
        this.lastBar = 0
      }

      let max = Math.max(0, ...this.bars)
      max = max > 0 ? max : 1
      let barScale = this.canvasHeight / max

      let barWidth = this.barWidth * this.scale
      if (!barWidth) {
        let w = this.canvasWidth
        barWidth = w / Math.max(this.bars.length, 1)
      }
      ctx.lineWidth = barWidth
      ctx.strokeStyle = '#ffc300'

      for (let i = this.lastBar; i < this.bars.length; i++) {
        ctx.beginPath()
        ctx.moveTo(i * barWidth, this.canvasHeight)
        ctx.lineTo(i * barWidth, this.canvasHeight - barScale * this.bars[i])
        ctx.stroke()
      }
      this.lastBar = this.bars.length
    }
  }
}
</script>

<style lang="scss">
.bargraph {
  display: flex;
  flex-direction: column;
}
.bargraph__canvas {
  display: flex;
  height: 100%;
  border-left: 1px solid #191919;
  border-bottom: 1px solid #191919;
}
.bargraph__label {
  font-size: 0.8em;
}
</style>
