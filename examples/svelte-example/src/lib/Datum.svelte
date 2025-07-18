<script>
  import VertretungsTabelle from "./VertretungsTabelle.svelte"

  export let datum;
  export let color;

  let pretty_datum = new Date(datum.datum).toLocaleString('de-DE', {
    weekday: 'long',
    day: 'numeric',
    month: 'long' 
  }
  );

  document.documentElement.style.setProperty('--color', color);
</script>

<div class="Datum">
    <div class="datum_container">
      {pretty_datum}
    </div>
    <br/>
    <VertretungsTabelle vertretungen={datum.vertretungen} color={color}/>
</div>

<style>
    .Datum {
      display: flex;
      flex-direction: column;
    }
    .datum_container {
        position: relative;
        align-self: flex-end;
        float: right;
        width: max-content;
        color: black;
        font-size: 0.8rem;
        border-width: 0.15rem;
        border-color: var(--color);
        border-style: solid;
        border-bottom-left-radius: 0.5rem;
        border-top: none;
        border-right: none;
        padding: 0.2rem;
    }
</style>
