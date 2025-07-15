<script>
  export let datum;
  export let color;
</script>

<div class="Datum">
    <div class="datum_container">
        {datum.datum}
    </div>
    <br/>
    <VertretungsTabelle vertretungen="{datum.vertretungen}" color="{color}"/>
</div>
