<template>
  <div>
    <h1>Players</h1>
    <table v-if="players.length > 0">
      <tr v-for="player in players" :key="'row'+player.id">
        <td> {{player.name}} </td>
        <td> {{player.score || 0}} </td>
      </tr>
    </table>
  </div>
</template>

<script>

export default {
  name: 'ScoreBoard',
  props: {
    players: Array.required
  }
}
</script>

<style scoped>
  table {
    margin: auto;
    width: 50%;
    border: 3px solid black;
  }
  h1 {

  }
</style>
