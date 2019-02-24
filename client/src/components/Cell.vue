<template>
  <div v-if="digit > 0">
    {{ digit }}
  </div>
</template>

<script>
export default {
  name: 'Cell',
  props: {
    digit: Number
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
