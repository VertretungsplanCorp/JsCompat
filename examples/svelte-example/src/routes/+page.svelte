<script lang="ts">
  import { Server } from "../../../../Cargo.toml"; // stattdessen git submodules benutzen
  import KlassenKachel from "$lib/KlassenKachel.svelte";

  let dati: Array<Object> = $state(new Array);

  let url = $state('http://vpapi.josewa.com');
  let klasse = $state('12e');
  function getData() {
    let server = new Server(url); 
    server.getKlasse("10c").then((req_dati: Object) => {
      console.log("Vertretungen: ", req_dati);
      dati = req_dati.dati;
    });
  }
  
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1><strong>Hallo beim Beispiel der des Vp_Js_Compat</strong></h1>

	<h2>
		zu sehen in <strong>src/routes/+page.svelte</strong>
	</h2>
  <br>
  <h2>
    Abfrage:
  </h2>
  <form>
    <label >
      server url:
      <input type="url" bind:value={url}/>
    </label>
    <br>
    <label>
      klasse:
      <input type="text" bind:value={klasse}/>
    </label>
    <br>
    <button onclick={() => {getData()}}>abrufen</button>
  </form>

  <h2>Kachel Klasse {klasse}:</h2>
  {#if dati.length > 0}
    <KlassenKachel klasse={klasse} dati={dati} />
  {/if}
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}
</style>
