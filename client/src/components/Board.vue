<template>
  <div>
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
import Cell from "./Cell.vue";
import {isDigitValid} from '../App.vue'

export default {
  name: "Board",
  props: {
    board: Array,
    sendDigit: Function
  },
  methods: {
    isDigitValid: ({irow, icol, board}) => digit => isDigitValid({irow, icol, board, digit})
  },
  components: {
    Cell
  }
};
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
</style>
