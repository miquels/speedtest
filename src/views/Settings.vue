<template>
<div>
  <div class="row">

    <div class="col-md-6 col-xs-12 stretchflex">
    <div class="doborder">
    <div class="row">

    <div class="col-xs-12"><p><b>Connection Type</b></p></div>
    <div class="col-xs-12">
      <div class="dropdown" v-dropdown>
        <button class="btn btn-secondary dropdown-toggle smallfont" type="button" id="dropdownMenuButton" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
          {{ dropName }}
        </button>
        <div class="dropdown-menu smallfont">
          <a class="dropdown-item" @click.prevent="drop('VDSL')" href="#">VDSL</a>
          <a class="dropdown-item" @click.prevent="drop('ADSL')" href="#">ADSL</a>
          <a class="dropdown-item" @click.prevent="drop('Cable')" href="#">Cable</a>
          <a class="dropdown-item" @click.prevent="drop('Fiber')" href="#">Fiber</a>
          <a class="dropdown-item" @click.prevent="drop('Custom')" href="#">Custom</a>
        </div>
      </div>
    </div>

    <p>&nbsp;</p>
    <div class="col-xs-12"><p><b>Display in</b></p></div>
    <div class="col-xs-12 smallfont">
      <div class="form-radio">
        <input class="magic-radio" type="radio" name="units" id="units-Mbps" value="units-Mbps" checked><label for="units-Mbps">Mbit/s</label>
      </div>
      <div class="form-radio">
        <input class="magic-radio" type="radio" name="units" id="units-MBps" value="units-MBps"><label for="units-MBps">MByte/s </label>
      </div>
    </div>
    <div class="col-xs-12 smallfont align-bottom">
      <div class="form-check">
        <input class="magic-checkbox" type="checkbox" id="overhead-MB" value="" checked><label for="overhead-MB">exclude overhead for MByte/s</label>
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
          <input class="form-control" type="text" value="8" id="upload-conns">
        </div>
      </div>
      <div class="form-group row">
        <label for="download-conns" class="col-xs-4 col-form-label">Download:</label>
        <div class="col-xs-8">
          <input class="form-control" type="text" value="8" id="download-conns">
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
        <input class="magic-checkbox" type="checkbox" id="l3-tcpip" value=""><label for="l3-tcpip">TCP/IP</label>
      </div>
      <div class="form-check">
        <input class="magic-checkbox" type="checkbox" id="l2-ppp" value=""><label for="l2-ppp">PPP</label>
      </div>
      <div class="form-check">
        <input class="magic-checkbox" type="checkbox" id="l2-vlan" value=""><label for="l2-vlan">VLAN</label>
      </div>
      <div class="form-check">
        <input class="magic-checkbox" type="checkbox" id="l2-ether" value="" checked><label for="l2-ether">Ethernet</label>
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
        <input class="magic-radio" type="radio" name="l2-proto" id="l2-vdsl" value="l2-vdsl"><label for="l2-vdsl">VDSL</label>
      </div>
      <div class="form-radio">
        <input class="magic-radio" type="radio" name="l2-proto" id="l2-adsl" value="l2-adsl"><label for="l2-adsl">ADSL</label>
      </div>
      <div class="form-radio">
        <input class="magic-radio" type="radio" name="l2-proto" id="l2-cable" value="l2-cable"><label for="l2-cable">Cable</label>
      </div>
      <div class="form-radio">
        <input class="magic-radio" type="radio" name="l2-proto" id="l2-fiber" value="l2-fiber" checked><label for="l2-fiber">Fiber (none)</label>
      </div>
    </div>

    </div>
    </div>
    </div>

  </div>
</div>
</template>

<script>
export default {
  name: 'settings',
  data: () => ({
    dropName: 'Generic'
  }),
  methods: {
    drop (t) {
      this.dropName = t
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
