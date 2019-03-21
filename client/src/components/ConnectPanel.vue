<template>
  <div :class="registered ? 'hide' : ''">
    <span class="unselectable">Your Name:</span>
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
    onClickConnect: Function,
    registered: Boolean
  },
  methods: {
    onClick: function () {
      this.onClickConnect({ name: this.name })
    }
  }
}
</script>

<style scoped>
.hide {
  visibility: hidden;
}
.unselectable {
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  -khtml-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}
</style>
