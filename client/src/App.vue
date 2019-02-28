<template>
  <div id="app">
    <Board :board="board" :sendDigit="sendDigit"/>
  </div>
</template>

<script>
import Board from './components/Board.vue'
import boards from './boards'

const emptyCell = () => ({value: null})
const givenCell = value => ({value, owner: 'given'})

const parseBoard = board => board
  .match(/.{1,9}/g)
  .map(row => row.split('')
    .map(d => {
      const value = parseInt(d)
      return value === 0 ? emptyCell() : givenCell(value)
    }))

const board = parseBoard(boards[Math.floor(Math.random()*boards.length)]);

export const isDigitValid = ({board, irow, icol, digit}) => {
  const rowValues = board[irow].map(cell => cell.value)
  const colValues = board.map((row, irow) => row[icol].value)

  return !rowValues.some(val => val === digit) &&
    !colValues.some(val => val === digit)
}

const sendDigit = ({irow, icol}) => digit => {
  board[irow][icol].value = digit
}

export default {
  name: 'app',
  data: () => ({ board }),
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
