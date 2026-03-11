<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  
  let path = $state("/home/aaron")
  let fileList:String[] = $state([]);

  async function read_dir(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    fileList = await invoke("read_files_from_path",{ path });
    console.log(fileList);
  }
</script>

<main class="container">
  <h1>Displaying contents of {path}</h1>

  <button onclick={read_dir}>read files</button>

  <ul>
  {#each fileList as file}
    <li>{file}</li>

  {/each}
  </ul>

</main>

<style>


</style>
