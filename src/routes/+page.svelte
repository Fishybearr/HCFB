<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  interface FileObject 
  {
    path:string,
    file_type:boolean
  }
  
  let path = $state("")
  let history:string[] = [];
  let fileList:FileObject[] = $state([]);

  //read initial home directory of user
  read_home_path();

  async function read_home_path()
  {
    path = await invoke<string>("read_home_dir");
  }

  async function read_parent_dir()
  {
    path = await invoke<string>("get_parent_dir", {path});
  }

  /*
  async function read_dir(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    fileList = await invoke<FileObject[]>("read_files_from_path",{ path });
    //console.log(fileList);
  }
  */

  async function load_file_data()
  {
    try
    {
      fileList = await invoke<FileObject[]>("read_files_from_path",{path});
    }
    catch(err)
    {
      console.log(err)
    }
    
  }

  
  function GoBack()
  {

    if(history.length > 0)
    {
      const prev = history.pop()

      if(prev)
      {
        path = prev;
      }
    } 
    
  }
    

  $effect(() =>
  {
    load_file_data();
    console.log("file path updated");
    
  } 
  );
</script>

<main class="container">
  <h1>Displaying contents of {path}</h1>

  <ul>
  <li><button onclick={GoBack}>{"<<"}</button></li>
  <li><button onclick={read_parent_dir}>../</button></li>

  {#each fileList as file}
    <li><button onclick={ () => { //TODO: update this section to validate a path before adding to history
    //Currently validates the next path while adding the current path to history regardless of it's filetype
      if(file.file_type){history.push(path)} console.log(history); path = file.path }}>{file.path}, {file.file_type}</button></li>

  {/each}
  </ul>

</main>

<style>


</style>
