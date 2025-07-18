<script>
    import Vertretung from "./Vertretung.svelte";

    export let vertretungen;
    export let color;
</script>

<table>
    <tbody style="width: 100%;">
    {#each vertretungen as vertretung}
      <tr style="width: 100%;">
          <Vertretung vertretung={vertretung} color={color} />
      </tr>
    {/each}
    </tbody>
</table>

<style>
  table {
      margin: 0.5rem;
      flex-grow: 1;
  }
</style>
