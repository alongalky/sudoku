<template>
  <div>
    Your Name:
    <input type="text" v-model="name" v-on:keyup.enter="onClick" />
    <button v-on:click="onClick">Connect</button>
  </div>
</template>

<script>

export default {
  name: 'ConnectPanel',
  data: () => ({
    name: ''
  }),
  props: {
    onClickConnect: Function
  },
  methods: {
    onClick: function () {
      this.onClickConnect({ name: this.name })
    }
  }
}
</script>

<style scoped>
</style>
