<template>
<div>
  <div class="row">

    <div class="col-md-6 col-xs-12 stretchflex">
    <div class="doborder">
    <div class="row">

    <div class="col-xs-12"><p><b>Connection Type</b></p></div>
    <div class="col-xs-12">
      <div class="dropdown" v-dropdown>
        <button class="btn btn-secondary dropdown-toggle smallfont"
         type="button" id="dropdownMenuButton" data-toggle="dropdown">
          {{ connType }}
        </button>
        <div class="dropdown-menu smallfont">
          <a class="dropdown-item" @click.prevent="connType='Generic'">Generic</a>
          <a class="dropdown-item" @click.prevent="connType='VDSL'">VDSL</a>
          <a class="dropdown-item" @click.prevent="connType='ADSL'"">ADSL</a>
          <a class="dropdown-item" @click.prevent="connType='Cable'">Cable</a>
          <a class="dropdown-item" @click.prevent="connType='Fiber'">Fiber</a>
          <a class="dropdown-item" @click.prevent="connType='Custom'">Custom</a>
        </div>
      </div>
    </div>

    <p>&nbsp;</p>
    <div class="col-xs-12"><p><b>Display in</b></p></div>
    <div class="col-xs-12 smallfont">
      <div class="form-radio">
        <input class="magic-radio" type="radio" name="units" id="units-Mbps"
         v-model="units" value="Mbps">
         <label for="units-Mbps">Mbit/s</label>
      </div>
      <div class="form-radio">
        <input class="magic-radio" type="radio" name="units" id="units-MBps"
         v-model="units" value="MBps">
        <label for="units-MBps">MByte/s </label>
      </div>
    </div>
    <div class="col-xs-12 smallfont align-bottom">
      <div class="form-check">
        <input class="magic-checkbox" type="checkbox" id="overhead-MB"
         v-model="MBexcludeOverhead">
        <label for="overhead-MB">exclude overhead for MByte/s</label>
      </div>
    </div>

    </div>
    </div>
    </div>

    <div class="col-md-6 col-xs-12 stretchflex">
    <div class="doborder">
    <div class="row">

    <div class="col-xs-12"><p><b>Connections</b></p></div>
    <div class="col-xs-12 smallfont">
      <div class="form-group row">
        <label for="upload-conns" class="col-xs-4 col-form-label">Upload:</label>
        <div class="col-xs-8">
          <input class="form-control" type="number"
           v-model="connsUp" id="upload-conns">
        </div>
      </div>
      <div class="form-group row">
        <label for="download-conns" class="col-xs-4 col-form-label">Download:</label>
        <div class="col-xs-8">
          <input class="form-control" type="number"
           v-model="connsDown" id="download-conns">
        </div>
      </div>
    </div>

    </div>
    </div>
    </div>

    <div class="col-md-6 col-xs-12 stretchflex">
    <div class="doborder">
    <div class="row">

    <div class="col-xs-12"><p><b>Layer 2/3 overhead</b></p></div>
    <div class="col-xs-12 smallfont">
      <div class="form-check">
        <input class="magic-checkbox" type="checkbox" id="tcpip"
         value="tcpip" v-model="l23overhead"><label for="tcpip">TCP/IP</label>
      </div>
      <div class="form-check">
        <input class="magic-checkbox" type="checkbox" id="ppp"
         value="ppp" v-model="l23overhead"><label for="ppp">PPP</label>
      </div>
      <div class="form-check">
        <input class="magic-checkbox" type="checkbox" id="vlan"
         value="vlan" v-model="l23overhead"><label for="vlan">VLAN</label>
      </div>
      <div class="form-check">
        <input class="magic-checkbox" type="checkbox" id="ether"
         value="ether" v-model="l23overhead"><label for="ether">Ethernet</label>
      </div>
    </div>

    </div>
    </div>
    </div>

    <div class="col-md-6 col-xs-12 stretchflex">
    <div class="doborder">
    <div class="row">

    <div class="col-xs-12"><p><b>Layer 1/2  overhead</b></p></div>
    <div class="col-xs-12 smallfont">
      <div class="form-radio">
        <input class="magic-radio" type="radio" id="vdsl"
         value="vdsl" v-model="l12overhead"><label for="vdsl">VDSL</label>
      </div>
      <div class="form-radio">
        <input class="magic-radio" type="radio" id="adsl"
         value="adsl" v-model="l12overhead"><label for="adsl">ADSL</label>
      </div>
      <div class="form-radio">
        <input class="magic-radio" type="radio" id="cable"
         value="cable" v-model="l12overhead"><label for="cable">Cable</label>
      </div>
      <div class="form-radio">
        <input class="magic-radio" type="radio" id="fiber"
         value="fiber" v-model="l12overhead"><label for="fiber">Fiber</label>
      </div>
    </div>

    </div>
    </div>
    </div>

    <div class="col-xs-12 smallfont">
      <div class="ml-1">display Mbit/s = {{ overhead | toFixed }} * 8 * Mbyte/s</div>
    </div>
  </div>
</div>
</template>

<script>
export default {
  name: 'settings',
  data: () => ({
    connType: null,
    units: null,
    MBexcludeOverhead: null,
    connsUp: null,
    connsDown: null,
    l23overhead: null,
    l12overhead: null,
    overhead: null
  }),
  filters: {
    'toFixed': (val) => val.toFixed(2)
  },
  created () {
    // copy state from store to this.$data
    let s = this.$store.state
    for (let i in this.$data) {
      if (typeof s[i] !== 'undefined') {
        this.$data[i] = s[i]
      }
    }
  },
  beforeDestroy () {
    // copy state from this.$data to store
    this.$store.commit('saveSettings', this.$data)
  },
  mounted () {
    this.$nextTick(() => { this.connType = this.$store.state.connType })
  },
  computed: {
    compoundProperty () {
      this.l12overhead
      this.l23overhead
      this.units
      this.MBexcludeOverhead
      return Date.now()
    }
  },
  watch: {
    compoundProperty () {
      this.calcOverhead()
    },
    connType (val) {
      this.updatingConnType = true
      this.setDefaults(val)
    }
  },
  methods: {
    setDefaults (how) {
      switch (how) {
        case 'Generic':
          this.l12overhead = 'fiber'
          this.l23overhead = [ 'ether' ]
          break
        case 'VDSL':
          this.l12overhead = 'vdsl'
          this.l23overhead = [ 'ether', 'vlan', 'ppp' ]
          break
        case 'ADSL':
          this.l12overhead = 'adsl'
          this.l23overhead = [ 'ppp' ]
          break
        case 'Cable':
          this.l12overhead = 'cable'
          this.l23overhead = [ 'ether' ]
          break
        case 'Fiber':
          this.l12overhead = 'fiber'
          this.l23overhead = [ 'ether', 'vlan', 'ppp' ]
          break
      }
    },
    toFixed (val, arg) {
      return val.toFixed(arg)
    },
    calcOverhead () {
      let psize = 1500
      let more = 0
      let factor = 1
      let isEther = this.l23overhead.indexOf('ether') >= 0
      let isPPP = this.l23overhead.indexOf('ppp') >= 0
      this.l23overhead.forEach((l) => {
        switch (l) {
          case 'tcpip':
            psize -= 40
            more += 40
            break
          case 'ppp':
            if (isEther) {
              psize -= 8
              more += 8
            } else {
              // PPPoA probably
              more += 2
            }
            break
          case 'vlan':
            more += 4
            break
          case 'ether':
            more += 18
            break
        }
      })
      switch (this.l12overhead) {
        case 'vdsl':
          // VDSL PTM
          more += 5
          // VDSL half-a-frame
          more += 32
          // VDSL 65/64
          factor *= 65 / 64
          break
        case 'adsl':
          // ATM: 53/48 and half a cell overhead
          factor *= 53 / 48
          more += 26
          if (isEther) {
            // RFC2684 bridged/llc
            more += 4
          } else {
            if (isPPP) {
              // RFC2384 AAL5/vcmux
              more += 8
            }
          }
          break
        case 'cable':
          more += 6
          break
      }
      if (!this.updatingConnType) {
        this.connType = 'Custom'
      }
      this.updatingConnType = false
      let r = 1
      if (!this.MBexcludeOverhead || this.units === 'Mbps') {
        r = factor * (psize + more) / psize
      }
      this.overhead = r
      return r
    }
  }
}
</script>

<style lang="scss">
.smallfont {
  font-size: 0.8em;
}
.stretchflex {
  display: flex;
  align-items: stretch;
}
.doborder {
  margin: 10px;
  padding: 10px;
  border: 1px solid #cccccc;
  border-radius: 10px;
  width: 100%;
}
</style>
