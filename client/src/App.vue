<template>
  <div id="app">
    <Board :board="state.board" :sendDigit="sendDigit"/>
  </div>
</template>

<script>
const socket = new WebSocket("ws://localhost:3012");
socket.onmessage = function (event) {
  const incomingState = JSON.parse(event.data).state;
  if (incomingState) {
    state.board = parseBoard(incomingState);
  }
}

import Board from './components/Board.vue'

const emptyCell = () => ({value: null})
const givenCell = value => ({value, owner: 'given'})

const parseBoard = board => board
  .match(/.{1,9}/g)
  .map(row => row.split('')
    .map(d => {
      const value = parseInt(d)
      return value === 0 ? emptyCell() : givenCell(value)
    }))

const emptyBoard = parseBoard('000000000000000000000000000000000000000000000000000000000000000000000000000000000');
const state = {
  board: emptyBoard
};

export const isDigitValid = ({board, irow, icol, digit}) => {
  const rowValues = board[irow].map(cell => cell.value)
  const colValues = board.map(row => row[icol].value)

  return !rowValues.some(val => val === digit) &&
    !colValues.some(val => val === digit)
}

const sendDigit = ({irow, icol}) => digit => {
  // state.board[irow][icol].value = digit
  socket.send(JSON.stringify({
    type: 'send_digit',
    irow,
    icol,
    digit
  }));
}

export default {
  name: 'app',
  data: () => ({ state }),
  components: {
    Board
  },
  methods: {
    sendDigit,
    isDigitValid
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
