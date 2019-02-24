<template>
  <div id="app">
    <Board :board="board"/>
  </div>
</template>

<script>
import Board from './components/Board.vue'
import boards from './boards'

const parseBoard = board => board
  .match(/.{1,9}/g)
  .map(row => row.split('').map(d => parseInt(d)))

const board = parseBoard(boards[Math.floor(Math.random()*boards.length)]);

export default {
  name: 'app',
  data: () => ({ board }),
  components: {
    Board
  }
}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
