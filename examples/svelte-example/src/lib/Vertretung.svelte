<script>
    export let vertretung;
    export let color;
    document.documentElement.style.setProperty('--color', color);
</script>

<table style="width: 100%;">
    <tbody style="width: 100%;">
    <tr style="width: 100%;">
        <td style="width: 10%">{vertretung.stunde}</td>
        <td style="width: 20%">{vertretung.fach}</td>
        <td style="width: 20%">{vertretung.lehrer}</td>
        <td style="width: 50%">{vertretung.raum}</td>
    </tr>
    {#if (vertretung.info !== "")}
        <tr class="info">
            <td colspan="4">{vertretung.info}</td>
        </tr>
    {/if}
    </tbody>
</table>

<style>
    * {
        margin: 0;
        padding: 0;
        border-collapse: collapse;
    }

    td {
        border-width: 0.1rem;
        color: black;
        border-color: var(--color);
        font-size: 0.75rem;
        border-style: solid;
        border-bottom: none;
        border-top: none;
    }

    table {
        border-width: 0.15rem;
        border-color: var(--color);
        border-style: solid;
    }

    .info {
        border-width: 0.1rem;
        border-color: var(--color);
        border-top-style: dashed;
    }
</style>
