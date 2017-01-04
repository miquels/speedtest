<script>
export default {
  name: 'resizemixin',

  created () {
    this.$resizeMixin = {}
  },

  mounted () {
    let curSize = {
      width: this.$el.clientWidth,
      height: this.$el.clientHeight
    }
    let r = () => {
      if (this.$resizeMixin.timer ||
          (this.$el.clientWidth === curSize.width &&
           this.$el.clientHeight === curSize.height)) {
        return
      }
      curSize.width = this.$el.clientWidth
      curSize.height = this.$el.clientHeight
      this.$resizeMixin.timer = window.setTimeout(() => {
        this.$emit('resize')
        this.$resizeMixin.timer = null
      }, 50)
    }
    this.$resizeMixin.handler = window.addEventListener('resize', r)
  },

  beforeDestroy () {
    window.clearTimeout(this.$resizeMixin.timer)
    window.removeEventListener('resize', this.$resizeMixin.handler)
  }
}
</script>
