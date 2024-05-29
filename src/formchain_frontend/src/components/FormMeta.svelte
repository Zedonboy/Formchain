<script>
  import { form_metadata, selected_context } from "$lib/store";

  let formInputs = [
    {
      name: "Header",
      svg: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6">
  <path d="M18.75 12.75h1.5a.75.75 0 0 0 0-1.5h-1.5a.75.75 0 0 0 0 1.5ZM12 6a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 12 6ZM12 18a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 12 18ZM3.75 6.75h1.5a.75.75 0 1 0 0-1.5h-1.5a.75.75 0 0 0 0 1.5ZM5.25 18.75h-1.5a.75.75 0 0 1 0-1.5h1.5a.75.75 0 0 1 0 1.5ZM3 12a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 3 12ZM9 3.75a2.25 2.25 0 1 0 0 4.5 2.25 2.25 0 0 0 0-4.5ZM12.75 12a2.25 2.25 0 1 1 4.5 0 2.25 2.25 0 0 1-4.5 0ZM9 15.75a2.25 2.25 0 1 0 0 4.5 2.25 2.25 0 0 0 0-4.5Z" />
</svg>`,

      type: "header",
    },

    {
      name: "Footer",
      svg: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6">
  <path d="M18.75 12.75h1.5a.75.75 0 0 0 0-1.5h-1.5a.75.75 0 0 0 0 1.5ZM12 6a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 12 6ZM12 18a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 12 18ZM3.75 6.75h1.5a.75.75 0 1 0 0-1.5h-1.5a.75.75 0 0 0 0 1.5ZM5.25 18.75h-1.5a.75.75 0 0 1 0-1.5h1.5a.75.75 0 0 1 0 1.5ZM3 12a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 3 12ZM9 3.75a2.25 2.25 0 1 0 0 4.5 2.25 2.25 0 0 0 0-4.5ZM12.75 12a2.25 2.25 0 1 1 4.5 0 2.25 2.25 0 0 1-4.5 0ZM9 15.75a2.25 2.25 0 1 0 0 4.5 2.25 2.25 0 0 0 0-4.5Z" /></svg>`,

      type: "footer",
    },
  ];
  // export const onClose = () => {};
</script>

<div
  class="divide-y divide-gray-200 h-1/2 overflow-hidden rounded-lg bg-white shadow-lg"
>
  <div class="px-4 py-5 sm:px-6">
    <!-- Content goes here -->
    <!-- We use less vertical padding on card headers on desktop than on body sections -->
  </div>
  <div class="px-4 py-5 sm:p-6 grid grid-cols-4 gap-4">
    {#each formInputs as input, index}
      <div>
        <div class="col-span-1 flex rounded-md shadow-sm">
          <div
            class="flex w-16 flex-shrink-0 items-center justify-center bg-primary-500 rounded-l-md text-sm font-medium text-white"
          >
            {@html input.svg}
          </div>
          <button
            on:click={(e) => {
              let last_index = $form_metadata.length;
              let q_data = {
                title: "Title",
                desc: "Description",
                type: input.type,
                svg: input.svg,
              };
              if ($form_metadata == null) {
                let n_q = [q_data];
                form_metadata.set(n_q);
              } else {
                let n_q = [...$form_metadata, q_data];
                form_metadata.set(n_q);
                selected_context.set(last_index);
              }
            }}
            class="flex flex-1 items-center justify-between truncate rounded-r-md border-b border-r border-t border-gray-200 hover:bg-gray-100 bg-white"
          >
            <div class="flex-1 truncate px-4 py-2 text-sm">
              <p class="font-medium text-gray-900">{input.name}</p>
            </div>
            <div class="flex-shrink-0 pr-2">
              <button
                type="button"
                class="inline-flex h-8 w-8 items-center justify-center rounded-full bg-transparent bg-white text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
              >
                <span class="sr-only">Open options</span>
                <svg
                  class="h-5 w-5"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                  aria-hidden="true"
                >
                  <path
                    d="M10 3a1.5 1.5 0 110 3 1.5 1.5 0 010-3zM10 8.5a1.5 1.5 0 110 3 1.5 1.5 0 010-3zM11.5 15.5a1.5 1.5 0 10-3 0 1.5 1.5 0 003 0z"
                  />
                </svg>
              </button>
            </div>
          </button>
        </div>
      </div>
    {/each}

    <!-- Content goes here -->
  </div>
</div>
