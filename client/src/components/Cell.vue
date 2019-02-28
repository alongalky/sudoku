<template>
  <div v-if="digit.value !== null" class="cell">
    {{ digit.value }}
  </div>
  <div v-else class="empty cell">
    <input type="text" v-on:input="onInput" />
  </div>
</template>

<script>

const onInput = function (e) {
  const value = e.target.value.substr(0, 1)
  const digit = /^[1-9]$/.test(value) ? value : ''
  e.target.value = digit

  if (digit.length === 1) {
    const num = parseInt(digit)

    if (this.isDigitValid(num)) {
      this.sendDigit(num)
    } else {
      e.target.value = ''
    }
  }
}

export default {
  name: 'Cell',
  props: {
    digit: Object,
    sendDigit: Function,
    isDigitValid: Function
  },
  methods: {
    onInput
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.cell {
  height: 100%;
  width: 100%;
}

.empty input {
  width: 80%;
  height: 80%;
  margin: 0;
  padding: 0;
  size: 1;
  text-align: center;
}
</style>
