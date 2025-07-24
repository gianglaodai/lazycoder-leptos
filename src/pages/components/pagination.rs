use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Pagination(
    first_result: i64,
    total_entities: i64,
    #[prop(default = 5)] max_results: i64,
    #[prop(default = 5)] max_visible_pages: i64,
) -> impl IntoView {
    let total_pages = (total_entities + max_results - 1) / max_results;
    let current_page = (first_result / max_results) + 1;
    let show_prev = current_page > 1;
    let show_next = current_page < total_pages;

    let half_visible = max_visible_pages / 2;
    let mut start_page = (current_page - half_visible).max(1);
    let end_page = (start_page + max_visible_pages - 1).min(total_pages);

    if end_page - start_page + 1 < max_visible_pages {
        start_page = (end_page - max_visible_pages + 1).max(1);
    }

    view! {
        <nav class="flex items-center justify-between border-t border-gray-200 px-4 sm:px-0 mt-8">
            <div class="-mt-px flex w-0 flex-1">
                {if show_prev {
                    view! {
                        <A
                            href=format!("?first_result={}&max_results={}", (current_page - 2) * max_results, max_results)
                            attr:class="inline-flex items-center border-t-2 border-transparent pr-1 pt-4 text-sm font-medium text-gray-500 hover:border-gray-300 hover:text-gray-700"
                        >
                            <svg class="mr-3 h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M18 10a.75.75 0 01-.75.75H4.66l2.1 1.95a.75.75 0 11-1.02 1.1l-3.5-3.25a.75.75 0 010-1.1l3.5-3.25a.75.75 0 111.02 1.1l-2.1 1.95h12.59A.75.75 0 0118 10z" clip-rule="evenodd" />
                            </svg>
                            Previous
                        </A>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
            </div>

            <div class="md:-mt-px md:flex">
                {(start_page..=end_page).map(|page| {
                    let is_current = page == current_page;
                    let class = if is_current {
                        "inline-flex items-center border-t-2 border-indigo-500 px-4 pt-4 text-sm font-medium text-indigo-600"
                    } else {
                        "inline-flex items-center border-t-2 border-transparent px-4 pt-4 text-sm font-medium text-gray-500 hover:border-gray-300 hover:text-gray-700"
                    };

                    view! {
                        <A
                            href=format!("?first_result={}&max_results={}", (page-1) * max_results, max_results)
                            attr:class=class
                        >
                            {page}
                        </A>
                    }
                }).collect_view()}
            </div>

            <div class="-mt-px flex w-0 flex-1 justify-end">
                {if show_next {
                    view! {
                        <A
                            href=format!("?first_result={}&max_results={}", current_page * max_results, max_results)
                            attr:class="inline-flex items-center border-t-2 border-transparent pl-1 pt-4 text-sm font-medium text-gray-500 hover:border-gray-300 hover:text-gray-700"
                        >
                            Next
                            <svg class="ml-3 h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M2 10a.75.75 0 01.75-.75h12.59l-2.1-1.95a.75.75 0 111.02-1.1l3.5 3.25a.75.75 0 010 1.1l-3.5 3.25a.75.75 0 11-1.02-1.1l2.1-1.95H2.75A.75.75 0 012 10z" clip-rule="evenodd" />
                            </svg>
                        </A>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
            </div>
        </nav>
    }
}
