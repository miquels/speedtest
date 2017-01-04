<template>
<div class="progressbar" :class="{'progressbar__stripes': waiting || value===null}">
  <div class="progressbar__p" :style="{width: widthpct}"></div>
</div>
</template>

<script>
export default {
  name: 'progressbar',
  props: {
    waiting: { type: [ Number, Boolean ], default: false },
    value: { type: Number, default: 0 },
    maxValue: { type: Number, default: 1 }
  },
  computed: {
    widthpct () {
      let val = this.value > 0 ? this.value : 0
      val = val > this.maxValue ? this.maxValue : val
      return Math.floor(100 * val / this.maxValue) + '%'
    }
  }
}
</script>

<style lang="scss">
.progressbar {
  position: relative;
  padding: 0px;
}
.progressbar__p {
  position: relative;
  margin: 0px;
  padding: 0px;
  height: 100%;
  background: currentcolor;
}
.progressbar__stripes {
  background-size: 30px 30px;
  background-image: linear-gradient(
    135deg, 
    rgba(black, 0.1)  25%, 
    transparent       25%, 
    transparent       50%, 
    rgba(black, 0.1)  50%, 
    rgba(black, 0.1)  75%, 
    transparent       75%, 
    transparent
  );
  animation: progressbar__a 0.8s linear infinite;
}
@keyframes progressbar__a {
  from { background-position: 0 0; }
  to   { background-position: 60px 30px; }
}
</style>
