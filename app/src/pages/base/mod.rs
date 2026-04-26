use crate::components::{
    hooks::use_theme_mode::ThemeMode,
    sidenav::SideNav,
    ui::{
        breadcrumb::{
            Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator,
        },
        button::{Button, ButtonVariant},
        empty::*,
        theme_toggle::ThemeToggle,
        table::*,
        dropdown_menu::*,
        alert_dialog::*,
    },
};
use components::{CreateTableDialog, CreateFieldDialog};
use icons::{Ellipsis, FolderCode, Lock, Plus, Trash};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use server::{create_table_in_base, get_base_tables, get_table_data, update_record_cell, delete_record, create_record, create_field, TableData};
use leptos::portal::Portal;

mod components;
pub mod server;

#[component]
pub fn BasePage() -> impl IntoView {
    let theme = ThemeMode::init();
    let params = use_params_map();
    let base_id = move || params.with(|p| p.get("id").unwrap_or_default());

    let (refresh_count, set_refresh_count) = signal(0);
    let tables = Resource::new(
        move || (base_id(), refresh_count.get()),
        |(id, _)| async move { get_base_tables(id).await },
    );

    let (selected_table_id, set_selected_table_id) = signal::<Option<String>>(None);

    let create_table_action = Action::new(move |name: &String| {
        let name = name.clone();
        let id = base_id();
        async move { create_table_in_base(id, name).await }
    });

    Effect::new(move |_| {
        if let Some(Ok(table)) = create_table_action.value().get() {
            set_refresh_count.update(|n| *n += 1);
            set_selected_table_id.set(Some(table.id));
        }
    });

    Effect::new(move |_| {
        if selected_table_id.get().is_none() {
            if let Some(Ok(list)) = tables.get() {
                if !list.is_empty() {
                    set_selected_table_id.set(Some(list[0].id.clone()));
                }
            }
        }
    });

    view! {
        <div
            class:dark=move || theme.is_dark()
            class="flex min-h-screen bg-[var(--background)] text-[var(--foreground)]"
        >
            <SideNav />
            <div class="flex-1 relative flex flex-col min-w-0">
                <div class="absolute top-4 right-4 p-2 z-50">
                    <ThemeToggle />
                </div>

                <div class="flex flex-col gap-4 p-8 pb-0">
                    <Breadcrumb>
                        <BreadcrumbList>
                            <BreadcrumbItem>
                                <BreadcrumbLink attr:href="/dashboard">"Dashboard"</BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem>
                                <BreadcrumbLink attr:href=format!(
                                    "/base/{}",
                                    base_id(),
                                )>"Base"</BreadcrumbLink>
                            </BreadcrumbItem>
                        </BreadcrumbList>
                    </Breadcrumb>

                    <div class="flex gap-4 justify-between items-center">
                        <div class="flex items-center gap-4 overflow-x-auto pb-2 scrollbar-hide">
                            <Suspense>
                                {move || {
                                    tables
                                        .get()
                                        .map(|res| {
                                            match res {
                                                Ok(list) => {
                                                    list.into_iter()
                                                        .map(|table| {
                                                            let id = table.id.clone();
                                                            let is_selected = move || {
                                                                selected_table_id.get() == Some(id.clone())
                                                            };
                                                            view! {
                                                                <button
                                                                    class=move || {
                                                                        format!(
                                                                            "px-4 py-2 text-sm font-medium border-b-2 transition-colors whitespace-nowrap {}",
                                                                            if is_selected() {
                                                                                "border-primary text-primary"
                                                                            } else {
                                                                                "border-transparent text-muted-foreground hover:text-foreground"
                                                                            },
                                                                        )
                                                                    }
                                                                    on:click=move |_| {
                                                                        set_selected_table_id.set(Some(table.id.clone()));
                                                                    }
                                                                >
                                                                    {table.name}
                                                                </button>
                                                            }
                                                                .into_any()
                                                        })
                                                        .collect_view()
                                                        .into_any()
                                                }
                                                _ => view! {}.into_any(),
                                            }
                                        })
                                }}
                            </Suspense>
                            <CreateTableDialog
                                title=move || {
                                    if create_table_action.pending().get() {
                                        view! { <Lock /> }.into_any()
                                    } else {
                                        view! { <Plus class="size-4" /> }.into_any()
                                    }
                                }
                                create_action=create_table_action
                            />
                        </div>
                    </div>
                </div>

                <div class="flex-1 overflow-auto p-8">
                    {move || {
                        if let Some(table_id) = selected_table_id.get() {
                            let b_id = base_id();
                            view! {
                                <TableGrid
                                    base_id=b_id
                                    table_id=table_id
                                />
                            }
                                .into_any()
                        } else {
                            view! {
                                <Suspense>
                                    {move || {
                                        tables
                                            .get()
                                            .map(|res| {
                                                if let Ok(list) = res {
                                                    if list.is_empty() {
                                                        view! {
                                                            <Empty class="flex-1 flex items-center justify-center">
                                                                <EmptyHeader>
                                                                    <EmptyMedia variant=EmptyMediaVariant::Icon>
                                                                        <FolderCode />
                                                                    </EmptyMedia>
                                                                    <EmptyTitle>"No Table Yet"</EmptyTitle>
                                                                    <EmptyDescription>
                                                                        "This base is empty. Create your first table to start organizing data! :3"
                                                                    </EmptyDescription>
                                                                </EmptyHeader>

                                                                <EmptyContent>
                                                                    <div class="flex gap-2">
                                                                        <CreateTableDialog
                                                                            title="Create Table".into_any()
                                                                            create_action=create_table_action
                                                                        />
                                                                    </div>
                                                                </EmptyContent>
                                                            </Empty>
                                                        }
                                                            .into_any()
                                                    } else {
                                                        view! {}.into_any()
                                                    }
                                                } else {
                                                    view! {}.into_any()
                                                }
                                            })
                                    }}
                                </Suspense>
                            }
                                .into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
fn TableGrid(base_id: String, table_id: String) -> impl IntoView {
    let base_id_sv = StoredValue::new(base_id);
    let table_id_sv = StoredValue::new(table_id);

    let table_data_res = Resource::new(
        move || (base_id_sv.get_value(), table_id_sv.get_value()),
        |(b_id, t_id)| async move { get_table_data(b_id, t_id).await },
    );

    let local_data = RwSignal::new(None::<TableData>);

    // Update local data when resource finishes
    Effect::new(move |_| {
        if let Some(Ok(data)) = table_data_res.get() {
            local_data.set(Some(data));
        }
    });

    let create_record_action = Action::new(move |_: &()| {
        let b_id = base_id_sv.get_value();
        let t_id = table_id_sv.get_value();
        async move { create_record(b_id, t_id).await }
    });

    let create_field_action = Action::new(move |name: &String| {
        let name = name.clone();
        let b_id = base_id_sv.get_value();
        let t_id = table_id_sv.get_value();
        async move { create_field(b_id, t_id, name).await }
    });

    Effect::new(move |_| {
        if let Some(Ok(new_record)) = create_record_action.value().get() {
            local_data.update(|data| {
                if let Some(d) = data {
                    d.records.push(new_record);
                }
            });
        }
    });

    Effect::new(move |_| {
        if let Some(Ok(new_field)) = create_field_action.value().get() {
            local_data.update(|data| {
                if let Some(d) = data {
                    d.fields.push(new_field);
                }
            });
        }
    });

    let on_delete_record = move |rid: String| {
        local_data.update(|data| {
            if let Some(d) = data {
                d.records.retain(|r| r.id != rid);
            }
        });
    };

    let on_update_cell = move |rid: String, fname: String, val: String| {
        local_data.update(|data| {
            if let Some(d) = data {
                if let Some(r) = d.records.iter_mut().find(|r| r.id == rid) {
                    r.cells.insert(fname, val);
                }
            }
        });
    };

    view! {
        <Suspense fallback=|| view! { <p>"Loading table data..."</p> }>
            {move || {
                let data_opt = local_data.get().or_else(|| {
                    table_data_res.get().and_then(|r| r.ok())
                });

                if let Some(data) = data_opt {
                    let fields_sv = StoredValue::new(data.fields.clone());
                    let column_count = data.fields.len() + 1;
                    view! {
                        <TableWrapper class="w-full">
                            <Table class="w-full max-w-none">
                                <TableHeader>
                                    <TableRow>
                                        {fields_sv.get_value()
                                            .into_iter()
                                            .map(|field| {
                                                view! {
                                                    <TableHead class="font-bold border-r last:border-r-0">
                                                        {field.name.clone()}
                                                    </TableHead>
                                                }
                                            })
                                            .collect_view()}
                                        <TableHead class="w-10 p-0 text-center">
                                            <div class="flex items-center justify-center h-full border-l">
                                                <CreateFieldDialog
                                                    title=view! { <Plus class="size-4" /> }.into_any()
                                                    create_action=create_field_action
                                                />
                                            </div>
                                        </TableHead>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>
                                    <For
                                        each=move || data.records.clone()
                                        key=|record| record.id.clone()
                                        let:record
                                    >
                                        {
                                            let record_id = record.id.clone();
                                            let record_cells = record.cells.clone();
                                            view! {
                                                <TableRow>
                                                    {fields_sv.get_value()
                                                        .into_iter()
                                                        .map({
                                                            let record_id = record_id.clone();
                                                            let record_cells = record_cells.clone();
                                                            move |field| {
                                                                let field_name = field.name.clone();
                                                                let value = record_cells
                                                                    .get(&field_name)
                                                                    .cloned()
                                                                    .unwrap_or_default();
                                                                let rid_for_cell = record_id.clone();
                                                                let rid_for_callback = record_id.clone();
                                                                let fname_for_cell = field_name.clone();
                                                                let fname_for_callback = field_name.clone();
                                                                view! {
                                                                    <EditableCell
                                                                        base_id=base_id_sv.get_value()
                                                                        table_id=table_id_sv.get_value()
                                                                        record_id=rid_for_cell
                                                                        field_name=fname_for_cell
                                                                        initial_value=value
                                                                        on_success=Callback::new({
                                                                            let rid = rid_for_callback.clone();
                                                                            let fname = fname_for_callback.clone();
                                                                            move |v| on_update_cell(rid.clone(), fname.clone(), v)
                                                                        })
                                                                    />
                                                                }
                                                            }
                                                        })
                                                        .collect_view()}
                                                    <TableCell class="p-0 text-center border-l">
                                                        <RecordActions
                                                            base_id=base_id_sv.get_value()
                                                            table_id=table_id_sv.get_value()
                                                            record_id=record_id.clone()
                                                            on_delete=Callback::new(move |rid| on_delete_record(rid))
                                                        />
                                                    </TableCell>
                                                </TableRow>
                                            }
                                        }
                                    </For>
                                    <TableRow
                                        class="cursor-pointer group/add-row hover:bg-muted/50 transition-colors"
                                        on:click=move |_| {
                                            create_record_action.dispatch(());
                                        }
                                    >
                                        <TableCell class="p-0 border-t" attr:colspan=column_count>
                                            <div class="flex items-center gap-2 px-3 py-2 text-muted-foreground group-hover/add-row:text-foreground transition-colors h-10">
                                                <Plus class="size-4" />
                                                <span>"Add Record"</span>
                                            </div>
                                        </TableCell>
                                    </TableRow>
                                </TableBody>
                            </Table>
                        </TableWrapper>
                    }.into_any()
                } else if let Some(Err(e)) = table_data_res.get() {
                    view! { <p class="text-destructive">{format!("Error: {}", e)}</p> }.into_any()
                } else {
                    view! { <p>"Preparing table..."</p> }.into_any()
                }
            }}
        </Suspense>
    }
}

#[component]
fn EditableCell(
    base_id: String,
    table_id: String,
    record_id: String,
    field_name: String,
    initial_value: String,
    on_success: Callback<String>,
) -> impl IntoView {
    let (is_editing, set_is_editing) = signal(false);
    let (value, set_value) = signal(initial_value.clone());
    let (pending, set_pending) = signal(false);

    let save_action = Action::new(
        move |(b_id, t_id, r_id, f_name, val): &(String, String, String, String, String)| {
            let b_id = b_id.clone();
            let t_id = t_id.clone();
            let r_id = r_id.clone();
            let f_name = f_name.clone();
            let val = val.clone();
            async move { update_record_cell(b_id, t_id, r_id, f_name, val).await }
        },
    );

    let base_id = StoredValue::new(base_id);
    let table_id = StoredValue::new(table_id);
    let record_id = StoredValue::new(record_id);
    let field_name = StoredValue::new(field_name);
    let initial_value_sv = StoredValue::new(initial_value);

    let handle_save = move || {
        let current_val = value.get();
        if current_val != initial_value_sv.get_value() {
            set_pending.set(true);
            save_action.dispatch((
                base_id.get_value(),
                table_id.get_value(),
                record_id.get_value(),
                field_name.get_value(),
                current_val,
            ));
        }
        set_is_editing.set(false);
    };

    let handle_save_sv = StoredValue::new(handle_save);

    Effect::new(move |_| {
        if let Some(res) = save_action.value().get() {
            set_pending.set(false);
            match res {
                Ok(_) => {
                    on_success.run(value.get_untracked());
                }
                Err(e) => {
                    println!("Error saving cell: {:?}", e);
                    set_value.set(initial_value_sv.get_value());
                }
            }
        }
    });

    view! {
        <TableCell
            class="p-0 border-r last:border-r-0 h-10 group relative"
            on:click=move |_| set_is_editing.set(true)
        >
            {move || {
                if is_editing.get() {
                    view! {
                        <input
                            class="absolute inset-0 w-full h-full px-3 py-0 outline-none border-none ring-0 bg-neutral-100 dark:bg-neutral-800 focus:bg-neutral-200 dark:focus:bg-neutral-700 transition-colors"
                            prop:value=move || value.get()
                            on:input=move |ev| set_value.set(event_target_value(&ev))
                            on:blur=move |_| handle_save_sv.get_value()()
                            on:keydown=move |ev: web_sys::KeyboardEvent| {
                                if ev.key() == "Enter" {
                                    handle_save_sv.get_value()();
                                } else if ev.key() == "Escape" {
                                    set_value.set(initial_value_sv.get_value());
                                    set_is_editing.set(false);
                                }
                            }
                            autofocus="true"
                        />
                    }
                        .into_any()
                } else {
                    view! {
                        <div class="px-3 py-0 w-full h-full flex items-center overflow-hidden whitespace-nowrap text-ellipsis">
                            <span class=move || if pending.get() { "opacity-50" } else { "" }>
                                {move || value.get()}
                            </span>
                        </div>
                    }
                        .into_any()
                }
            }}
        </TableCell>
    }
}

#[component]
fn RecordActions(
    base_id: String,
    table_id: String,
    record_id: String,
    on_delete: Callback<String>,
) -> impl IntoView {
    let base_id_sv = StoredValue::new(base_id);
    let table_id_sv = StoredValue::new(table_id);
    let record_id_sv = StoredValue::new(record_id);

    let delete_record_action = Action::new(
        move |(b_id, t_id, r_id): &(String, String, String)| {
            let b_id = b_id.clone();
            let t_id = t_id.clone();
            let r_id = r_id.clone();
            async move { delete_record(b_id, t_id, r_id).await }
        },
    );

    Effect::new(move |_| {
        if let Some(Ok(_)) = delete_record_action.value().get() {
            on_delete.run(record_id_sv.get_value());
        }
    });

    view! {
        <DropdownMenu>
            <DropdownMenuTrigger class="p-0 w-8 h-8 flex items-center justify-center border-0 bg-transparent hover:bg-accent rounded-none">
                <Ellipsis class="size-4" />
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-[160px]">
                <DropdownMenuLabel>"Actions"</DropdownMenuLabel>
                <DropdownMenuGroup class="mt-2">
                    <DropdownMenuItem class="p-0">
                        <AlertDialog class="w-full">
                            <AlertDialogTrigger class="w-full flex items-center gap-2 px-2 py-1.5 text-sm text-destructive hover:bg-destructive/10">
                                <Trash class="size-4" />
                                <span>"Delete"</span>
                            </AlertDialogTrigger>
                            <Portal>
                                <AlertDialogContent class="w-[425px]">
                                    <AlertDialogBody>
                                        <AlertDialogHeader>
                                            <AlertDialogTitle>"Are you sure?"</AlertDialogTitle>
                                            <AlertDialogDescription>
                                                "This action cannot be undone. This will permanently delete this record."
                                            </AlertDialogDescription>
                                        </AlertDialogHeader>
                                        <AlertDialogFooter>
                                            <AlertDialogClose>"Cancel"</AlertDialogClose>
                                            <Button
                                                variant=ButtonVariant::Destructive
                                                on:click={
                                                    move |_| {
                                                        delete_record_action
                                                            .dispatch((
                                                                base_id_sv.get_value(),
                                                                table_id_sv.get_value(),
                                                                record_id_sv.get_value(),
                                                            ));
                                                    }
                                                }
                                            >
                                                "Delete"
                                            </Button>
                                        </AlertDialogFooter>
                                    </AlertDialogBody>
                                </AlertDialogContent>
                            </Portal>
                        </AlertDialog>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
