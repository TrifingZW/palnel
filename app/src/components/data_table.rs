use leptos::prelude::*;

/// 列定义。
#[derive(Clone)]
pub struct ColumnDef {
    pub key: String,
    pub label: String,
}

/// 现代通用数据表格，支持点击表头排序与多选。
#[component]
pub fn DataTable(
    columns: Vec<ColumnDef>,
    #[prop(into)] rows: Signal<Vec<Vec<String>>>,
    #[prop(optional)] selected: Option<RwSignal<Vec<usize>>>,
) -> impl IntoView {
    let (sort_col, set_sort_col) = signal(None::<usize>);
    let (sort_asc, set_sort_asc) = signal(true);
    let inner_selected = RwSignal::new(Vec::<usize>::new());
    let selected = selected.unwrap_or(inner_selected);
    let columns = StoredValue::new(columns);

    let toggle_sort = move |col: usize| {
        set_sort_col.update(|cur| {
            if *cur == Some(col) {
                set_sort_asc.update(|asc| *asc = !*asc);
            } else {
                *cur = Some(col);
                set_sort_asc.set(true);
            }
        });
    };

    view! {
        <div class="data-table">
            <div class="data-table__header">
                <span class="data-table__cell data-table__cell--check">
                    <Checkbox
                        checked=move || {
                            let data = rows.get();
                            let sel = selected.get();
                            !data.is_empty() && data.len() == sel.len()
                        }
                        on_click=move |_| {
                            let data = rows.get();
                            let sel = selected.get();
                            if !data.is_empty() && data.len() == sel.len() {
                                selected.set(Vec::new());
                            } else {
                                selected.set((0..data.len()).collect());
                            }
                        }
                    />
                </span>
                {move || {
                    columns.get_value().iter().enumerate().map(|(i, col)| {
                        let label = col.label.clone();
                        let is_sorted = move || sort_col.get() == Some(i);
                        view! {
                            <span
                                class="data-table__cell data-table__cell--header"
                                class:data-table__cell--sorted=is_sorted
                                on:click=move |_| toggle_sort(i)
                            >
                                {move || {
                                    let arrow = if is_sorted() {
                                        if sort_asc.get() { " ▲" } else { " ▼" }
                                    } else {
                                        ""
                                    };
                                    format!("{}{}", label, arrow)
                                }}
                            </span>
                        }
                    }).collect_view()
                }}
            </div>
            <div class="data-table__body">
                {move || {
                    let mut data = rows.get();
                    if let Some(col) = sort_col.get() {
                        let asc = sort_asc.get();
                        data.sort_by(|a, b| {
                            let va = a.get(col).map(String::as_str).unwrap_or("");
                            let vb = b.get(col).map(String::as_str).unwrap_or("");
                            let cmp = va.cmp(vb);
                            if asc { cmp } else { cmp.reverse() }
                        });
                    }
                    data.into_iter().enumerate().map(|(i, row)| {
                        view! {
                            <div class="data-table__row" class:data-table__row--selected=move || selected.get().contains(&i)>
                                <span class="data-table__cell data-table__cell--check">
                                    <Checkbox
                                        checked=move || selected.get().contains(&i)
                                        on_click=move |_| {
                                            selected.update(|sel| {
                                                if let Some(pos) = sel.iter().position(|&x| x == i) {
                                                    sel.remove(pos);
                                                } else {
                                                    sel.push(i);
                                                }
                                            });
                                        }
                                    />
                                </span>
                                {row.into_iter().map(|cell| {
                                    view! {
                                        <span class="data-table__cell">{cell}</span>
                                    }
                                }).collect_view()}
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

/// SVG 复选框。
#[component]
fn Checkbox(
    checked: impl Fn() -> bool + Send + Sync + 'static,
    #[prop(into)] on_click: Callback<()>,
) -> impl IntoView {
    let checked = std::sync::Arc::new(checked);

    let checked2 = std::sync::Arc::clone(&checked);
    let aria = move || checked().to_string();

    view! {
        <span class="checkbox" on:click=move |_| on_click.run(()) role="checkbox" aria-checked=aria>
            {move || {
                if checked2() {
                    view! {
                        <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
                            <path d="M14 1a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H2a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1h12zM2 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2H2z"/>
                            <path d="M10.97 4.97a.75.75 0 0 1 1.071 1.05l-3.992 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.235.235 0 0 1 .02-.022z"/>
                        </svg>
                    }.into_any()
                } else {
                    view! {
                        <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
                            <path d="M14 1a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H2a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1h12zM2 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2H2z"/>
                        </svg>
                    }.into_any()
                }
            }}
        </span>
    }
}
