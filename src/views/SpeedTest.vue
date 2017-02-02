<template>
<div>
  <!--div class="row height-2vh"></div-->
  <div class="row height-5vh hidden-sm-down"></div>

  <div class="row">
    <div class="col-xs-12 col-md-6 height-35vh height-40vh-sm-down">
      <div class="row height-100">
        <div class="col-xs-12">
          <radial-gauge class="gauge" :value="gaugeValue" :units="scale.units"
            :majorTicks="scale.majorTicks" :minorTicks="scale.minorTicks"
            :transformValue="scale.transformValue" :maxValue="scale.maxValue">
          </radial-gauge>
        </div>
      </div>
    </div>
    <div class="col-xs-12 col-md-6 height-30vh-sm-down height-35vh">
      <div class="row height-50">
        <div class="col-xs-8 flex">
          <bar-graph class="bars" :bars="downBars" label="download" :barWidth="2"></bar-graph>
        </div>
        <div class="col-xs-4 flex">
          <number-value class="number" :label="scale.units" :value="downFinal"></number-value>
        </div>
      </div>
      <div class="row height-50">
        <div class="col-xs-8 flex">
          <bar-graph class="bars" :bars="upBars" label="upload" :barWidth="2"></bar-graph>
        </div>
        <div class="col-xs-4 flex">
          <number-value class="number" :label="scale.units" :value="upFinal"></number-value>
        </div>
      </div>
    </div>
  </div>

  <div class="row height-10vh hidden-sm-down"></div>

  <div class="row flex-items-xs-middle height-10vh height-20vh-sm-down">
    <div class="col-xs-12 col-md-10">
      <progress-bar class="progress" :value="progress" :waiting="waiting"></progress-bar>
    </div>
    <div class="col-xs-12 col-md-2">
      <button class="btn btn-block-sm-down float-xs-right" :class="{'btn-warning': !running, 'btn-danger': running}" @click="toggle">{{ running ? 'Stop' : 'Start' }}</button>
    </div>
  </div>

  <div v-if="false && development" class="devel">
  *development debug*<br>
  downloaded: {{ curBytes }} bytes<br>
  speed: {{ curRate }} MB/s<br>
  elapsed: {{ curSecs }} secs<br>
  <span v-if="avgSpeed">Average: {{ avgSpeed }} MB/s</span></br>
  </div>
</div>
</template>

<script>
import RadialGauge from 'components/RadialGauge'
import BarGraph from 'components/BarGraph'
import NumberValue from 'components/NumberValue'
import ProgressBar from 'components/ProgressBar'
import ResizeMixin from 'components/ResizeMixin'
import SpeedTest from '../lib/speedtest.js'
import Config from 'config'

const Mbitprops = {
  majorTicks: [ 0, 1, 5, 20, 50, 100, 200, 500, 1000 ],
  minorTicks: [
    [ 0, 1, 0.5 ], [ 1, 5, 1 ], [ 5, 20, 5 ], [ 20, 50, 10 ],
    [ 50, 100, 25 ], [ 100, 200, 50 ], [ 200, 1000, 100 ]
  ],
  transformValue: function (val) { return Math.log(val + 1) * 100 / Math.log(1001) },
  maxValue: 1000,
  units: 'Mbit/s'
}
const MByteprops = {
  majorTicks: [ 0, 0.1, 0.5, 2, 5, 10, 20, 50, 100 ],
  minorTicks: [
    [ 0, 0.1, 0.05 ], [ 0.1, 0.5, 0.1 ], [ 0.5, 2, 0.5 ], [ 2, 5, 1 ],
    [ 5, 10, 2.5 ], [ 10, 20, 5 ], [ 20, 100, 10 ]
  ],
  transformValue: function (val) { return Math.log(val * 10 + 1) * 100 / Math.log(1001) },
  maxValue: 100,
  units: 'MByte/s'
}

export default {
  name: 'speedtest',
  data: () => ({
    gaugeValue: -1,
    curBytes: 0,
    curRate: 0,
    curSecs: 0,
    avgSpeed: 0,
    upBars: [],
    downBars: [],
    upFinal: '--.-',
    downFinal: '--.-',
    running: false,
    stopping: false,
    progress: 0,
    waiting: false,
    maxTestMS: 10000,
    development: Config.development,
    scale: Mbitprops
  }),

  components: {
    BarGraph,
    RadialGauge,
    NumberValue,
    ProgressBar
  },

  mixins: [
    ResizeMixin
  ],

  created () {
    this.timeout = null
    window.ST = this
    let u = this.$store.state.units
    this.scale = u === 'Mbps' ? Mbitprops : MByteprops
  },

  mounted () {
    this.$on('resize', () => {
      this.$children.forEach(c => { c.$emit('resize') })
    })
  },

  beforeDestroy () {
    this.stopTest()
  },

  methods: {
    startTest () {
      this.upFinal = '--.-'
      this.downFinal = '--.-'
      this.upBars = []
      this.downBars = []
      this.isUp = false
      this.running = true
      this.stopping = false
      this.progress = 0
      this.waiting = true
      this.startTest1().then(() => {
        window.clearTimeout(this.timeout)
        if (this.stopping) {
          this.running = false
          return
        }
        // this.waiting = true
        this.isUp = true
        this.startTest1().then(() => {
          window.clearTimeout(this.timeout)
          this.running = false
          this.progress = this.stopping ? 0 : 1
        })
      }).catch(() => {
        window.clearTimeout(this.timeout)
        this.running = false
        this.progress = 0
        this.waiting = false
      })
    },

    startTest1 () {
      return new Promise((resolve, reject) => {
        this.avgSpeed = 0
        this.curSecs = 0
        this.curBytes = 0
        this.curRate = 0

        let s = this.$store.state
        let baseUrl = 'ws://' + s.config.apihost.default + ':' +
                s.config.apiport + '/speedtest/'
        let url = this.isUp ? baseUrl + 'sink' : baseUrl + 'source'
        this.st = new SpeedTest({
          url: url,
          isUpload: this.isUp,
          maxTestMS: 10000,
          numParallel: this.isUp ? s.connsUp : s.connsDown
        })
        this.gaugeValue = 0
        this.st.connect().then(() => {
          this.timeout = window.setTimeout(this.updateProgress, 100)
          return this.st.runTest()
        }).then(() => {
          console.log('curbytes', this.curBytes, 'cursecs', this.curSecs)
          if (this.isUp) {
            this.upFinal = this.gaugeValue.toFixed(1)
          } else {
            this.downFinal = this.gaugeValue.toFixed(1)
          }
          this.avgSpeed = ((this.curBytes / this.curSecs) / 1000000).toFixed(1)
          this.waiting = !this.stopping
          this.gaugeValue = -1
          window.setTimeout(resolve, 500)
        }).catch((e) => {
          console.log('SpeedTest error:', e)
          this.gaugeValue = -1
          window.setTimeout(reject, 500)
        })
      })
    },

    stopTest () {
      this.stopping = true
      this.progress = 0
      this.waiting = false
      if (this.st) {
        this.st.stop()
      }
    },

    toggle () {
      if (!this.running) {
        this.startTest()
      } else {
        this.stopTest()
      }
    },

    updateProgress () {
      if (!this.running || this.gaugeValue < 0) {
        return
      }
      let r = this.st.poll()
      if (r.curSecs > 0.8) {
        Object.assign(this, r)
        let s = this.$store.state
        let f = s.overhead * (s.units === 'Mbps' ? 8 : 1)
        let curbps = this.curRate * f
        let avgbps = this.avgRate * f
        let bars
        if (this.isUp) {
          bars = this.upBars
        } else {
          bars = this.downBars
        }
        bars.push(curbps)
        this.gaugeValue = avgbps
        this.waiting = false
        this.progress = (r.curSecs - 0.8) / (this.maxTestMS / 500)
        if (this.progress > 0.5) {
          this.progress = 0.5
        }
        if (this.isUp) {
          this.progress += 0.5
        }
        console.log('spd', curbps)
      }
      this.timeout = window.setTimeout(this.updateProgress, 100)
    }
  }
}
</script>

<style lang="scss">
.devel {
  background: #c00000;
  color: white;
  margin-top: 20px;
}
.gauge {
  position: relative;
  width: 100%;
  height: 100%;
}
.bars {
  flex: 3;
  width: 200px;
  height: 50px;
  color: black;
  align-self: flex-end;
}
.number {
  flex: 2;
}
.progress {
  border: 1px solid black;
  border-radius: 999px;
  color: #ffc300;
  height: 10px;
  width: 100%;
  margin: auto;
}
.btn-warning, .btn-warning:hover, .btn-warning:active, .btn-warning:visited {
    background-color: #ffc300 !important;
}
</style>
