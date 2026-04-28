use crate::components::{
    hooks::use_theme_mode::ThemeMode,
    sidenav::SideNav,
    ui::{
        breadcrumb::{
            Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator,
        },
        button::{Button, ButtonSize, ButtonVariant},
        context_menu::{
            ContextMenu, ContextMenuContent, ContextMenuGroup, ContextMenuItem, ContextMenuLabel,
            ContextMenuTrigger,
        },
        data_table::{
            DataTable, DataTableBody, DataTableCell, DataTableHead, DataTableHeader, DataTableRow,
            DataTableWrapper,
        },
        empty::*,
        theme_toggle::ThemeToggle,
    },
};
use components::{CreateFieldDialog, CreateTableDialog};
use icons::{
    AlignLeft, Calendar, Cpu, FolderCode, Globe, Hash, Link, List, Lock, Mail, Phone, Plus,
    Settings, Trash, Type, User,
};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use server::*;

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
        let id = params.with_untracked(|p| p.get("id").unwrap_or_default());
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
                                <BreadcrumbLink attr:href=move || {
                                    format!("/base/{}", base_id())
                                }>"Base"</BreadcrumbLink>
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
                            view! { <TableGrid base_id=b_id table_id=table_id /> }.into_any()
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
fn FieldIcon(config: charac::models::field::FieldConfig) -> impl IntoView {
    use charac::models::field::{FieldConfig, TextConfig};
    match config {
        FieldConfig::Text(t) => match t {
            TextConfig::SingleLine { .. } => view! { <Type class="size-4" /> }.into_any(),
            TextConfig::LongText { .. } => view! { <AlignLeft class="size-4" /> }.into_any(),
            TextConfig::Email => view! { <Mail class="size-4" /> }.into_any(),
            TextConfig::URL => view! { <Globe class="size-4" /> }.into_any(),
            TextConfig::Phone => view! { <Phone class="size-4" /> }.into_any(),
        },
        FieldConfig::Number(_) => view! { <Hash class="size-4" /> }.into_any(),
        FieldConfig::Select(_) => view! { <List class="size-4" /> }.into_any(),
        FieldConfig::Datetime(_) => view! { <Calendar class="size-4" /> }.into_any(),
        FieldConfig::Relation(_) => view! { <Link class="size-4" /> }.into_any(),
        FieldConfig::User(_) => view! { <User class="size-4" /> }.into_any(),
        FieldConfig::Computed(_) => view! { <Cpu class="size-4" /> }.into_any(),
        FieldConfig::Custom(_) => view! { <Settings class="size-4" /> }.into_any(),
    }
}

#[component]
fn TableGrid(base_id: String, table_id: String) -> impl IntoView {
    let base_id_sv = StoredValue::new(base_id);
    let table_id_sv = StoredValue::new(table_id);
    let (refresh_counter, set_refresh_counter) = signal(0);

    let table_data_res = Resource::new(
        move || {
            (
                base_id_sv.get_value(),
                table_id_sv.get_value(),
                refresh_counter.get(),
            )
        },
        |(b_id, t_id, _)| async move { get_table_data(b_id, t_id).await },
    );

    let update_action = Action::new(move |params: &(String, String, String, String)| {
        let (record_id, field_name, new_value, base_id) = params.clone();
        let table_id = table_id_sv.get_value();
        async move { update_record_cell(base_id, table_id, record_id, field_name, new_value).await }
    });

    let create_field_action = Action::new(move |name: &String| {
        let name = name.clone();
        let b_id = base_id_sv.get_value();
        let t_id = table_id_sv.get_value();
        async move { create_field(b_id, t_id, name).await }
    });

    let delete_field_action = Action::new(move |field_id: &String| {
        let field_id = field_id.clone();
        let b_id = base_id_sv.get_value();
        let t_id = table_id_sv.get_value();
        async move { delete_field(b_id, t_id, field_id).await }
    });

    let create_record_action = Action::new(move |_: &()| {
        let b_id = base_id_sv.get_value();
        let t_id = table_id_sv.get_value();
        async move { create_record(b_id, t_id).await }
    });

    let delete_record_action = Action::new(move |record_id: &String| {
        let record_id = record_id.clone();
        let b_id = base_id_sv.get_value();
        let t_id = table_id_sv.get_value();
        async move { delete_record(b_id, t_id, record_id).await }
    });

    // Refresh table data after successful update
    Effect::new(move |_| {
        if update_action.value().with(|v| matches!(v, Some(Ok(_))))
            || create_field_action
                .value()
                .with(|v| matches!(v, Some(Ok(_))))
            || delete_field_action
                .value()
                .with(|v| matches!(v, Some(Ok(_))))
            || create_record_action
                .value()
                .with(|v| matches!(v, Some(Ok(_))))
            || delete_record_action
                .value()
                .with(|v| matches!(v, Some(Ok(_))))
        {
            set_refresh_counter.update(|n| *n += 1);
        }
    });

    // Automatic initialization for empty tables
    Effect::new(move |_| {
        if let Some(Ok(data)) = table_data_res.get() {
            if data.fields.is_empty() && !create_field_action.pending().get() {
                create_field_action.dispatch("Name".to_string());
            } else if !data.fields.is_empty() && data.records.is_empty() && !create_record_action.pending().get() {
                create_record_action.dispatch(());
            }
        }
    });

    view! {
        <Transition fallback=|| {
            view! { <p>"Loading table data..."</p> }
        }>
            {move || {
                table_data_res.get().map(|res| {
                    match res {
                        Ok(data) => {
                            let fields_sv = StoredValue::new(data.fields.clone());
                            let base_id_for_cells = base_id_sv.get_value();
                            view! {
                                <DataTableWrapper class="w-full border rounded-md">
                                    <DataTable class="w-full max-w-none border-collapse">
                                        <DataTableHeader>
                                            <DataTableRow>
                                                <DataTableHead class="w-10 p-0 text-center border-r">
                                                    "#"
                                                </DataTableHead>
                                                {fields_sv
                                                    .get_value()
                                                    .into_iter()
                                                    .map(|field| {
                                                        let field_id = field.id.clone();
                                                        view! {
                                                            <DataTableHead class="font-bold border-r last:border-r-0 min-w-[200px] p-0">
                                                                <ContextMenu>
                                                                    <ContextMenuTrigger class="flex items-center gap-2 w-full h-full px-4 hover:bg-muted/50 transition-colors cursor-context-menu">
                                                                        <FieldIcon config=field.config.clone() />
                                                                        {field.name.clone()}
                                                                    </ContextMenuTrigger>
                                                                    <ContextMenuContent>
                                                                        <ContextMenuLabel>"Field Actions"</ContextMenuLabel>
                                                                        <ContextMenuGroup>
                                                                            <CreateFieldDialog
                                                                                title=move || view! {
                                                                                    <div class="flex items-center w-full px-2 py-1.5 text-sm cursor-default hover:bg-accent hover:text-accent-foreground rounded-sm">
                                                                                        <Plus class="size-4 mr-2" />
                                                                                        "Add Field"
                                                                                    </div>
                                                                                }
                                                                                create_action=create_field_action
                                                                            />
                                                                            <ContextMenuItem on:click=move |_| {
                                                                                delete_field_action.dispatch(field_id.clone());
                                                                            }>
                                                                                <Trash class="size-4 mr-2 text-destructive" />
                                                                                <span class="text-destructive">"Delete Field"</span>
                                                                            </ContextMenuItem>
                                                                        </ContextMenuGroup>
                                                                    </ContextMenuContent>
                                                                </ContextMenu>
                                                            </DataTableHead>
                                                        }
                                                    })
                                                    .collect_view()}
                                                <DataTableHead class="w-10 p-0 text-center">
                                                    <CreateFieldDialog
                                                        title=move || view! {
                                                            <Button
                                                                variant=ButtonVariant::Ghost
                                                                size=ButtonSize::Icon
                                                            >
                                                                <Plus class="size-4" />
                                                            </Button>
                                                        }
                                                        create_action=create_field_action
                                                    />
                                                </DataTableHead>
                                            </DataTableRow>
                                        </DataTableHeader>
                                        <DataTableBody>
                                            <For
                                                each=move || data.records.clone()
                                                key=|record| record.id.clone()
                                                let:record
                                            >
                                                {
                                                    let record_cells = record.cells.clone();
                                                    let record_id = record.id.clone();
                                                    let base_id = base_id_for_cells.clone();
                                                    let record_id_for_delete = record_id.clone();
                                                    view! {
                                                        <DataTableRow class="group hover:bg-muted/50">
                                                            <DataTableCell class="w-10 p-0 text-center border-r text-xs text-muted-foreground">
                                                                <ContextMenu>
                                                                    <ContextMenuTrigger class="w-full h-full flex items-center justify-center cursor-context-menu">
                                                                        <List class="size-3 opacity-0 group-hover:opacity-100 transition-opacity" />
                                                                    </ContextMenuTrigger>
                                                                    <ContextMenuContent>
                                                                        <ContextMenuLabel>"Record Actions"</ContextMenuLabel>
                                                                        <ContextMenuGroup>
                                                                            <ContextMenuItem on:click=move |_| {
                                                                                create_record_action.dispatch(());
                                                                            }>
                                                                                <Plus class="size-4 mr-2" />
                                                                                "Add Record"
                                                                            </ContextMenuItem>
                                                                            <ContextMenuItem on:click=move |_| {
                                                                                delete_record_action.dispatch(record_id_for_delete.clone());
                                                                            }>
                                                                                <Trash class="size-4 mr-2 text-destructive" />
                                                                                <span class="text-destructive">"Delete Record"</span>
                                                                            </ContextMenuItem>
                                                                        </ContextMenuGroup>
                                                                    </ContextMenuContent>
                                                                </ContextMenu>
                                                            </DataTableCell>
                                                            {fields_sv
                                                                .get_value()
                                                                .into_iter()
                                                                .map({
                                                                    let record_cells = record_cells.clone();
                                                                    let record_id = record_id.clone();
                                                                    let base_id = base_id.clone();
                                                                    move |field| {
                                                                        let field_id = field.id.clone();
                                                                        let value = record_cells
                                                                            .get(&field_id)
                                                                            .cloned()
                                                                            .unwrap_or_default();
                                                                        let config = field.config.clone();
                                                                        let record_id = record_id.clone();
                                                                        let base_id = base_id.clone();
                                                                        view! {
                                                                            <DataTableCell class="px-0 py-0 h-10 border-r last:border-r-0">
                                                                                <EditableCell
                                                                                    value=value
                                                                                    config=config
                                                                                    record_id=record_id
                                                                                    field_name=field_id
                                                                                    base_id=base_id
                                                                                    update_action=update_action
                                                                                />
                                                                            </DataTableCell>
                                                                        }
                                                                    }
                                                                })
                                                                .collect_view()}
                                                            <DataTableCell class="w-10 p-0">""</DataTableCell>
                                                        </DataTableRow>
                                                    }
                                                }
                                            </For>
                                            <DataTableRow
                                                class="cursor-pointer hover:bg-muted/50 text-muted-foreground transition-colors"
                                                on:click=move |_| {
                                                    create_record_action.dispatch(());
                                                }
                                            >
                                                <DataTableCell
                                                    attr:colspan=move || fields_sv.get_value().len() + 2
                                                    class="h-10 px-4"
                                                >
                                                    <div class="flex items-center gap-2">
                                                        <Plus class="size-4" />
                                                        "New Record"
                                                    </div>
                                                </DataTableCell>
                                            </DataTableRow>
                                        </DataTableBody>
                                    </DataTable>
                                </DataTableWrapper>
                            }.into_any()
                        }
                        Err(e) => view! { <p class="text-destructive">{format!("Error: {}", e)}</p> }.into_any(),
                    }
                }).unwrap_or_else(|| view! { <p>"Preparing table..."</p> }.into_any())
            }}
        </Transition>

    }
}

#[component]
fn EditableCell(
    value: String,
    config: charac::models::field::FieldConfig,
    record_id: String,
    field_name: String,
    base_id: String,
    update_action: Action<(String, String, String, String), Result<(), ServerFnError>>,
) -> impl IntoView {
    use charac::models::{FieldConfig, TextConfig};

    let (is_editing, set_is_editing) = signal(false);
    let (edit_value, set_edit_value) = signal(value.clone());
    let (display_value, set_display_value) = signal(value);
    let (error_msg, set_error_msg) = signal::<Option<String>>(None);

    let input_type = match &config {
        FieldConfig::Number(_) => "number",
        FieldConfig::Text(t) => match t {
            TextConfig::Email { .. } => "email",
            TextConfig::URL { .. } => "url",
            TextConfig::Phone { .. } => "tel",
            _ => "text",
        },
        _ => "text",
    };

    let input_ref = NodeRef::<leptos::html::Input>::new();
    Effect::new(move |_| {
        if is_editing.get() {
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
            }
        }
    });

    let save = {
        let config = config.clone();
        let record_id = record_id.clone();
        let field_name = field_name.clone();
        let base_id = base_id.clone();
        move || {
            let val = edit_value.get();
            let validation_result = match &config {
                FieldConfig::Text(text_config) => match text_config {
                    TextConfig::SingleLine { .. } => {
                        if val.contains('\n') || val.contains('\r') {
                            Err("Single line only - no line breaks".to_string())
                        } else {
                            Ok(())
                        }
                    }
                    TextConfig::LongText { .. } => Ok(()),
                    TextConfig::Email => {
                        if !val.is_empty() && !validator::ValidateEmail::validate_email(&val) {
                            Err("Invalid email format".to_string())
                        } else {
                            Ok(())
                        }
                    }
                    TextConfig::URL => {
                        if !val.is_empty() && !validator::ValidateUrl::validate_url(&val) {
                            Err("Invalid URL format".to_string())
                        } else {
                            Ok(())
                        }
                    }
                    TextConfig::Phone => Ok(()),
                },
                FieldConfig::Number(_) => {
                    if !val.is_empty() && val.parse::<f64>().is_err() {
                        Err("Must be a valid number".to_string())
                    } else {
                        Ok(())
                    }
                }
                FieldConfig::Datetime(_) => {
                    if !val.is_empty() {
                        use chrono::DateTime;
                        if DateTime::parse_from_str(&val, "%Y-%M-%D").is_err() {
                            Err("Invalid datetime (use ISO 8601)".to_string())
                        } else {
                            Ok(())
                        }
                    } else {
                        Ok(())
                    }
                }
                _ => Ok(()),
            };
            if let Err(e) = validation_result {
                set_error_msg.set(Some(e));
            } else {
                set_error_msg.set(None);
                set_is_editing.set(false);
                if val != display_value.get() {
                    update_action.dispatch((
                        record_id.clone(),
                        field_name.clone(),
                        val,
                        base_id.clone(),
                    ));
                    set_display_value.set(edit_value.get());
                }
            }
        }
    };

    view! {
        <div class="relative w-full h-full min-h-[40px]">
            {move || {
                let save_for_blur = save.clone();
                let save_for_keydown = save.clone();
                if is_editing.get() {
                    view! {
                        <div class="absolute inset-0 z-10 flex flex-col">
                            <input
                                node_ref=input_ref
                                type=input_type
                                class="w-full h-full px-3 py-2 text-sm bg-background border-2 border-primary rounded-none focus:outline-none"
                                value=move || edit_value.get()
                                on:input=move |ev| {
                                    set_edit_value.set(event_target_value(&ev));
                                    set_error_msg.set(None);
                                }
                                on:blur=move |_| save_for_blur()
                                on:keydown=move |ev| {
                                    match ev.key().as_str() {
                                        "Enter" => {
                                            save_for_keydown();
                                        }
                                        "Escape" => {
                                            set_edit_value.set(display_value.get());
                                            set_error_msg.set(None);
                                            set_is_editing.set(false);
                                        }
                                        _ => {}
                                    }
                                }
                            />
                            {move || {
                                error_msg
                                    .get()
                                    .map(|err| {
                                        view! {
                                            <div class="absolute top-full left-0 right-0 bg-destructive text-destructive-foreground text-xs px-2 py-1 rounded-b shadow-lg z-20">
                                                {err}
                                            </div>
                                        }
                                    })
                            }}
                        </div>
                    }
                        .into_any()
                } else {
                    view! {
                        <div
                            class="w-full h-full px-3 py-2 text-sm cursor-pointer hover:bg-muted/80 transition-colors flex items-center overflow-hidden whitespace-nowrap text-ellipsis"
                            on:click=move |_| {
                                set_edit_value.set(display_value.get());
                                set_is_editing.set(true);
                            }
                        >
                            {move || {
                                let val = display_value.get();
                                if val.is_empty() {
                                    view! {
                                        <span class="text-muted-foreground italic">"Empty"</span>
                                    }
                                        .into_any()
                                } else {
                                    view! { <span>{val}</span> }.into_any()
                                }
                            }}
                        </div>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
