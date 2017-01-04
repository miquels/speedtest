<template>
<div class="numbervalue">
  <div class="numbervalue__num">{{ value }}</div>
  <div class="numbervalue__label" v-if="value">{{ label }}</div>
</div>
</template>

<script>
export default {
  name: 'numbervalue',

  props: {
    value: {},
    label: {}
  }
}
</script>

<style lang="scss">
.numbervalue {
  display: flex;
  flex-direction: column;
  margin-left: 5px;
}
.numbervalue__num {
  display: flex;
  flex: 1;
  align-items: flex-end;
  justify-content: flex-end;
  font-size: 2em;
}
.numbervalue__label {
  display: flex;
  align-items: flex-end;
  justify-content: flex-end;
  font-size: 0.8em;
  margin-left: 1em;
}
</style>
