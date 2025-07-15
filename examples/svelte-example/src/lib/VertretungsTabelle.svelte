<script>
    import Vertretung from "./Vertretung.svelte";

    export let vertretungen;
    export let color;
</script>

<table style="width: 100%;">
    <tbody style="width: 100%;">
    {#each vertretungen as vertretung}
    <tr style="width: 100%;">
        <Vertretung data="{vertretung}" color="{color}" />
    </tr>
    {/each}
    </tbody>
</table>
