<script>
  import FormCard from "../../components/FormCard.svelte";

</script>
<div
  class="border-2 border-dashed border-gray-300 flex-1 rounded-xl dark:border-gray-600 h-auto p-4"
>

  <button
    type="button"
    class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800"
    >
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6 inline">
        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
      </svg>
      Create Form</button
  >
  <div class="mt-4">
    <p class="text-2xl">Templates</p>
    <div>
        <h2 class="text-sm font-medium text-gray-500">Choose Template and build from there</h2>
        <ul role="list" class="mt-3 grid grid-cols-1 gap-5 sm:grid-cols-2 sm:gap-6 lg:grid-cols-4 ">
          <li class="col-span-1 flex rounded-md shadow-sm cursor-pointer hover:bg-gray-200 focus:ring-4 focus:ring-blue-300">
            <div class="flex w-16 flex-shrink-0 items-center justify-center bg-pink-600 rounded-l-md text-sm font-medium text-white">N</div>
            <div class="flex flex-1 items-center justify-between truncate rounded-r-md border-b border-r border-t border-gray-200 bg-inherit">
              <div class="flex-1 truncate px-4 py-2 text-sm">
                <a href="#" class="font-medium text-gray-900 hover:text-gray-600">Newsletter Template</a>
                <p class="text-gray-500">1 input</p>
              </div>
              
            </div>
          </li>
          
        </ul>
      </div>
      
  </div>
</div>
<div
  class="border-2 border-dashed border-gray-300 flex-1 rounded-xl dark:border-gray-600 min-h-96"
>
<div class="w-full h-full p-4 grid grid-cols-4 gap-3">
    <FormCard/>
    <FormCard/>
    <FormCard/>
    <FormCard/>
    <FormCard/>
    <FormCard/>
    <FormCard/>
    <FormCard/>
    <FormCard/>
    <FormCard/>
</div>
<!-- <div class="w-full h-96 flex justify-center items-center flex-col text-center">
    <svg class="h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
      <path vector-effect="non-scaling-stroke" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 13h6m-3-3v6m-9 1V7a2 2 0 012-2h6l2 2h6a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2z" />
    </svg>
    <h3 class="mt-2 text-sm font-semibold text-gray-900">No Forms</h3>
    <p class="mt-1 text-sm text-gray-500">Get started by creating a new Form</p>
    <div class="mt-6">
      <button type="button" class="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
        <svg class="-ml-0.5 mr-1.5 h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
          <path d="M10.75 4.75a.75.75 0 00-1.5 0v4.5h-4.5a.75.75 0 000 1.5h4.5v4.5a.75.75 0 001.5 0v-4.5h4.5a.75.75 0 000-1.5h-4.5v-4.5z" />
        </svg>
        New Form
      </button>
    </div>
  </div> -->
</div>
<!-- <div
  class="border-2 border-dashed border-gray-300 flex-1 rounded-xl dark:border-gray-600 h-96"
></div>
<div
  class="border-2 border-dashed border-gray-300 flex-1 rounded-xl dark:border-gray-600 h-96"
></div>
<div
  class="border-2 border-dashed border-gray-300 flex-1 rounded-xl dark:border-gray-600 h-96"
></div> -->
