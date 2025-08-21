use crate::pages::components::{
    Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink,
    PaginationNext, PaginationPrevious,
};
use leptos::prelude::*;

/// A high-level paginator that wraps the low-level Pagination components and
/// performs all page calculations internally.
///
/// Inputs:
/// - first_result: offset of the first entity on the current page (>= 0)
/// - total_entities: total number of entities available (>= 0)
/// - max_results: page size (> 0 ideally). If <= 0, treated as 1 to avoid div by zero.
/// - max_visible_pages: desired number of visible page items (including first/last and window). Minimum 5.
#[component]
pub fn Paginator(
    first_result: i64,
    total_entities: i64,
    #[prop(optional, default = 5)] max_results: i64,
    #[prop(optional, default = 7)] max_visible_pages: i64,
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    // Normalize inputs
    let page_size = if max_results <= 0 { 1 } else { max_results } as i64;
    let total = total_entities.max(0);

    let total_pages = ((total + page_size - 1) / page_size).max(1);
    let mut current_page = if page_size > 0 {
        first_result / page_size
    } else {
        0
    };
    if current_page >= total_pages {
        current_page = total_pages - 1;
    }

    // Prev/next offsets
    let prev_fr = if current_page > 0 {
        Some(first_result.saturating_sub(page_size))
    } else {
        None
    };
    let next_fr = if (current_page + 1) < total_pages {
        Some(first_result + page_size)
    } else {
        None
    };

    // Determine which numbered pages to show
    let max_visible = max_visible_pages.max(5) as i64; // ensure at least 5 for good UX

    // Helper to build hrefs consistent with existing pages
    let href_for = move |page_idx: i64| -> String {
        let first = page_idx * page_size;
        format!("?first_result={}&max_results={}", first, page_size)
    };

    // Build the list of page AnyView nodes
    let pages_view = {
        let mut vs: Vec<AnyView> = Vec::new();

        // If total_pages is small, just show all
        if total_pages <= max_visible {
            for i in 0..total_pages {
                let is_active = i == current_page;
                vs.push(
                    view! {
                        <PaginationItem>
                            <PaginationLink is_active=is_active href=href_for(i)>
                                {(i + 1).to_string()}
                            </PaginationLink>
                        </PaginationItem>
                    }
                    .into_any(),
                );
            }
            vs
        } else {
            // Always show first and last; show a middle window around current_page
            let window = 1i64; // show current-1, current, current+1 (like previous logic)
                               // You can widen the window based on max_visible if desired, but keep simple & consistent
                               // with previously implemented behavior.

            // First page
            vs.push(
                view! {
                    <PaginationItem>
                        <PaginationLink is_active={current_page==0} href=href_for(0)>
                            {"1"}
                        </PaginationLink>
                    </PaginationItem>
                }
                .into_any(),
            );

            // Left ellipsis if needed
            if current_page > (1 + window) {
                vs.push(view! { <PaginationEllipsis/> }.into_any());
            }

            // Middle pages: current_page-1, current_page, current_page+1 within bounds (excluding first/last)
            let start = (current_page.saturating_sub(window)).max(1);
            let end = ((current_page + window).min(total_pages - 2)).max(start);
            for i in start..=end {
                let is_active = i == current_page;
                vs.push(
                    view! {
                        <PaginationItem>
                            <PaginationLink is_active=is_active href=href_for(i)>
                                {(i + 1).to_string()}
                            </PaginationLink>
                        </PaginationItem>
                    }
                    .into_any(),
                );
            }

            // Right ellipsis if needed
            if current_page + 1 + window < (total_pages - 1) {
                vs.push(view! { <PaginationEllipsis/> }.into_any());
            }

            // Last page
            vs.push(view! {
                <PaginationItem>
                    <PaginationLink is_active={current_page==total_pages-1} href=href_for(total_pages-1)>
                        {total_pages.to_string()}
                    </PaginationLink>
                </PaginationItem>
            }.into_any());

            vs
        }
    };

    // Render the wrapper Pagination
    let class = class.unwrap_or_default();
    view! {
        <Pagination class=class>
            <PaginationContent>
                <PaginationItem>
                    {
                        match prev_fr {
                            Some(pf) => view! { <PaginationPrevious href=format!("?first_result={}&max_results={}", pf, page_size) /> }.into_any(),
                            None => view! { <PaginationPrevious /> }.into_any(),
                        }
                    }
                </PaginationItem>

                {view! {{pages_view}}}

                <PaginationItem>
                    {
                        match next_fr {
                            Some(nf) => view! { <PaginationNext href=format!("?first_result={}&max_results={}", nf, page_size) /> }.into_any(),
                            None => view! { <PaginationNext /> }.into_any(),
                        }
                    }
                </PaginationItem>
            </PaginationContent>
        </Pagination>
    }
}
