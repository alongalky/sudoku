<template>
  <div :class="`unselectable ${registered ? '' : 'disabled'}`">
    <div v-if="!registered" class='register-banner'>
      <div class="box">
        Register first!
      </div>
    </div>
    <ul id="example-1">
      <table class="sudoku-table">
        <tbody>
          <tr v-for="(row, irow) in board" :key="'row'+irow">
            <td v-for="(cell, icol) in row" :key="'row'+irow+'col'+icol">
              <Cell :digit="cell" :sendDigit="sendDigit({irow, icol})" :isDigitValid="isDigitValid({irow, icol, board})"/>
            </td>
          </tr>
        </tbody>
      </table>
    </ul>
  </div>
</template>

<script>
import Cell from './Cell.vue'
import { isDigitValid } from '../App.vue'

export default {
  name: 'Board',
  props: {
    board: Array,
    sendDigit: Function,
    registered: Boolean
  },
  methods: {
    isDigitValid: ({ irow, icol, board }) => digit => isDigitValid({ irow, icol, board, digit })
  },
  components: {
    Cell
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.sudoku-table {
  border: 3px solid black;
  margin-left: auto;
  margin-right: auto;
  font-size: 16pt;
}
.sudoku-table td {
  border: 1px solid black;
  width: 35px;
  height: 35px;
}
.unselectable {
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  -khtml-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}
.disabled {
  pointer-events:none;
  color: lightgray;
}
.disabled .sudoku-table {
  border: 3px solid lightgray;
}
.disabled .sudoku-table td {
  border: 1px solid lightgray;
}
.register-banner {
  position: absolute;
  width: 100%;
  top: 200px;
  left: 0px;
  text-align: center;
  font-size: 32pt;
  color: black;
}
.box {
  border: 3px solid black;
  padding: 20px;
  width: 30%;
  background-color: yellowgreen;
  margin: auto;
}
</style>
