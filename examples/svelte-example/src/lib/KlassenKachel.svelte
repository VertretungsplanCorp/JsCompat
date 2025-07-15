<script>
  export let klasse;
  export let dati;

  function getRandomVibrantColor() {
      // Hue: 0-360 (full color spectrum)
      const hue = Math.floor(Math.random() * 360);

      // Saturation: high for vibrant colors (e.g., 80-100%)
      const saturation = Math.floor(Math.random() * 20) + 80;

      // Lightness: medium range for vibrancy (e.g., 40-60%)
      const lightness = Math.floor(Math.random() * 20) + 40;

      // Return as HSL string
      return `hsl(${hue}, ${saturation}%, ${lightness}%)`;
  }

  let color = getRandomVibrantColor();
  import VertretungsTabelle from "./VertretungsTabelle.svelte";
  document.documentElement.style.setProperty('--color', color);
</script>

<div class="klassenKachel">
    <div class="klasse">{klasse}</div>
    <div class="kachelContent">
      {#each dati as datum}
          <Datum datum="{datum}" color="{color}"/>
      {/each}
    </div>
</div>

<style>
    .klassenKachel {
        width: 25vw;
    }

    .kachelContent {
        border-color: var(--color);
        border-style: solid;
        border-width: 0.25rem;
        border-radius: 1rem;
        background-color: white;
    }

    .heute {
        border-bottom-style: solid;
        border-bottom-color: var(--color);
        border-width: 0.2rem;
        padding: 0.2rem;
    }

    .morgen {
        padding: 0.2rem;
    }

    .datum {
        margin-top: -0.2rem;
        margin-right: -0.2rem;
        position: relative;
        float: right;
        width: max-content;
        color: black;
        font-size: 0.8rem;
        border-width: 0.15rem;
        border-color: black;
        border-style: solid;
        border-bottom-left-radius: 0.5rem;
        border-top: none;
        border-right: none;
        padding: 0.15rem;
    }

    .klasse {
        position: relative;
        left: -1.5rem;
        top: 1.5rem;
        width: 3rem;
        height: 3rem;
        line-height: 3rem;
        font-size: 1.5rem;
        border-radius: 50%;
        background-color: white;
        border-color: var(--color);
        color: var(--color);
        text-align: center;
        border-style: solid;
        border-width: 0.25rem;
    }
</style>
