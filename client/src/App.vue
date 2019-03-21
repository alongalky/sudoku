<template>
  <div id="app">
    <ConnectPanel :onClickConnect="onClickConnect" :registered="state.registered" />
    <Board :board="state.board" :sendDigit="sendDigit" :registered="state.registered"/>
    <ScoreBoard :players="state.players" />
  </div>
</template>

<script>
import Board from './components/Board.vue'
import ConnectPanel from './components/ConnectPanel.vue'
import ScoreBoard from './components/ScoreBoard.vue'

const socket = new WebSocket('ws://localhost:3012')
socket.onmessage = event => {
  const data = JSON.parse(event.data)
  const incomingState = data.state
  if (incomingState) {
    state.board = parseBoard(incomingState)
  }

  if (data.type === 'welcome') {
    state.playerId = data.playerId
    state.registered = true
  }

  if (data.type === 'update_players') {
    state.players = data.players
  }
}

const emptyCell = () => ({ value: null })
const givenCell = value => ({ value, owner: 'given' })

const parseBoard = board => board
  .match(/.{1,9}/g)
  .map(row => row.split('')
    .map(d => {
      const value = parseInt(d)
      return value === 0 ? emptyCell() : givenCell(value)
    }))

const emptyBoard = parseBoard('000000000000000000000000000000000000000000000000000000000000000000000000000000000')
const state = {
  registered: false,
  board: emptyBoard,
  players: [],
  playerId: null
}

export const isDigitValid = ({ board, irow, icol, digit }) => {
  const rowValues = board[irow].map(cell => cell.value)
  const colValues = board.map(row => row[icol].value)
  const boxLeftCol = icol - (icol % 3)

  const boxTopRow = irow - (irow % 3)
  const boxValues = [
    ...board[boxTopRow].slice(boxLeftCol, boxLeftCol + 3),
    ...board[boxTopRow + 1].slice(boxLeftCol, boxLeftCol + 3),
    ...board[boxTopRow + 2].slice(boxLeftCol, boxLeftCol + 3)
  ].map(cell => cell.value)

  return ![...rowValues, ...colValues, ...boxValues].some(val => val === digit)
}

const sendDigit = ({ irow, icol }) => digit => {
  socket.send(JSON.stringify({
    type: 'send_digit',
    irow,
    icol,
    digit
  }))
}

const onClickConnect = ({ name }) => {
  socket.send(JSON.stringify({
    type: 'connect',
    name
  }))
}

export default {
  name: 'app',
  data: () => ({ state }),
  components: {
    Board,
    ConnectPanel,
    ScoreBoard
  },
  methods: {
    sendDigit,
    isDigitValid,
    onClickConnect
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
