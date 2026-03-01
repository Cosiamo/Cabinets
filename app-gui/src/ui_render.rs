use std::cell::RefCell;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use app_core::filesystem;
use slint::{ComponentHandle, ModelRc, VecModel};

slint::include_modules!();

#[derive(Clone)]
struct VisibleNode {
    path: PathBuf,
    name: String,
    level: i32,
    is_folder: bool,
}

struct UiState {
    home_dir: PathBuf,
    current_root: PathBuf,
    sidebar_paths: Vec<PathBuf>,
    expanded_paths: HashSet<PathBuf>,
    visible_nodes: Vec<VisibleNode>,
    selected_sidebar: Option<usize>,
    selected_tree: Option<usize>,
    active_tab: i32,
}

impl UiState {
    fn new(home_dir: PathBuf) -> Self {
        Self {
            home_dir: home_dir.clone(),
            current_root: home_dir,
            sidebar_paths: Vec::new(),
            expanded_paths: HashSet::new(),
            visible_nodes: Vec::new(),
            selected_sidebar: None,
            selected_tree: None,
            active_tab: 0,
        }
    }
}

fn user_home_dir() -> PathBuf {
    env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/"))
}

fn refresh_sidebar(ui: &AppWindow, state: &mut UiState) {
    let folders = filesystem::list_folder_names(&state.home_dir).unwrap_or_default();

    state.sidebar_paths = folders
        .iter()
        .map(|name| state.home_dir.join(name))
        .collect();

    if state.selected_sidebar.is_none() && !state.sidebar_paths.is_empty() {
        state.selected_sidebar = Some(0);
        state.current_root = state.sidebar_paths[0].clone();
    }

    let selected = state.selected_sidebar;
    let sidebar_rows: Vec<SidebarFolder> = folders
        .into_iter()
        .enumerate()
        .map(|(index, name)| SidebarFolder {
            name: name.into(),
            selected: selected == Some(index),
        })
        .collect();

    ui.set_sidebar_folders(ModelRc::new(VecModel::from(sidebar_rows)));
}

fn push_visible_children(state: &UiState, path: &Path, level: i32, out: &mut Vec<VisibleNode>) {
    let folders = filesystem::list_folder_names(path).unwrap_or_default();
    for folder in folders {
        let folder_path = path.join(&folder);
        out.push(VisibleNode {
            path: folder_path.clone(),
            name: folder,
            level,
            is_folder: true,
        });

        if state.expanded_paths.contains(&folder_path) {
            push_visible_children(state, &folder_path, level + 1, out);
        }
    }

    let files = filesystem::list_file_names(path).unwrap_or_default();
    for file in files {
        out.push(VisibleNode {
            path: path.join(&file),
            name: file,
            level,
            is_folder: false,
        });
    }
}

fn build_tree_rows(state: &mut UiState) {
    let mut rows = Vec::new();
    push_visible_children(state, &state.current_root, 0, &mut rows);
    state.visible_nodes = rows;
}

fn apply_tab_filter(nodes: &[VisibleNode], tab: i32) -> Vec<VisibleNode> {
    match tab {
        1 => nodes.iter().filter(|n| n.is_folder).cloned().collect(),
        2 => nodes.iter().filter(|n| !n.is_folder).cloned().collect(),
        _ => nodes.to_vec(),
    }
}

fn refresh_tree(ui: &AppWindow, state: &mut UiState) {
    build_tree_rows(state);

    let filtered = apply_tab_filter(&state.visible_nodes, state.active_tab);
    state.visible_nodes = filtered.clone();
    let selected = state.selected_tree;
    let tree_rows: Vec<TreeRow> = filtered
        .iter()
        .enumerate()
        .map(|(index, n)| TreeRow {
            name: n.name.clone().into(),
            path: n.path.to_string_lossy().into_owned().into(),
            level: n.level,
            is_folder: n.is_folder,
            expanded: n.is_folder && state.expanded_paths.contains(&n.path),
            selected: selected == Some(index),
        })
        .collect();

    ui.set_tree_rows(ModelRc::new(VecModel::from(tree_rows)));
    ui.set_status_text(
        format!(
            "Root: {} | {} items",
            state.current_root.to_string_lossy(),
            filtered.len()
        )
        .into(),
    );
}

fn collect_search_matches(root: &Path, query: &str, level: i32, out: &mut Vec<VisibleNode>) {
    let folders = filesystem::list_folder_names(root).unwrap_or_default();
    for folder in folders {
        let path = root.join(&folder);
        if folder.to_lowercase().contains(query) {
            out.push(VisibleNode {
                path: path.clone(),
                name: folder.clone(),
                level,
                is_folder: true,
            });
        }
        collect_search_matches(&path, query, level + 1, out);
    }

    let files = filesystem::list_file_names(root).unwrap_or_default();
    for file in files {
        if file.to_lowercase().contains(query) {
            out.push(VisibleNode {
                path: root.join(&file),
                name: file,
                level,
                is_folder: false,
            });
        }
    }
}

fn run_search(ui: &AppWindow, state: &mut UiState, query: &str) {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        state.selected_tree = None;
        refresh_tree(ui, state);
        return;
    }

    let query_lower = trimmed.to_lowercase();
    let mut matches = Vec::new();
    collect_search_matches(Path::new("/"), &query_lower, 0, &mut matches);

    let tree_rows: Vec<TreeRow> = matches
        .iter()
        .enumerate()
        .map(|(index, n)| TreeRow {
            name: n.name.clone().into(),
            path: n.path.to_string_lossy().into_owned().into(),
            level: n.level,
            is_folder: n.is_folder,
            expanded: false,
            selected: state.selected_tree == Some(index),
        })
        .collect();

    state.visible_nodes = matches;
    ui.set_tree_rows(ModelRc::new(VecModel::from(tree_rows)));
    ui.set_status_text(
        format!(
            "Search '/' for '{}' -> {} matches",
            trimmed,
            state.visible_nodes.len()
        )
        .into(),
    );
}

pub fn render_ui() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let state = Rc::new(RefCell::new(UiState::new(user_home_dir())));
    ui.set_active_tab(0);

    {
        let mut state = state.borrow_mut();
        refresh_sidebar(&ui, &mut state);
        refresh_tree(&ui, &mut state);
    }

    ui.on_sidebar_folder_selected({
        let ui_handle = ui.as_weak();
        let state = state.clone();
        move |index| {
            let ui = ui_handle.unwrap();
            let mut state = state.borrow_mut();
            let idx = index as usize;

            if idx >= state.sidebar_paths.len() {
                return;
            }

            state.selected_sidebar = Some(idx);
            state.current_root = state.sidebar_paths[idx].clone();
            state.selected_tree = None;
            state.expanded_paths.clear();

            refresh_sidebar(&ui, &mut state);
            refresh_tree(&ui, &mut state);
        }
    });

    ui.on_tree_row_selected({
        let ui_handle = ui.as_weak();
        let state = state.clone();
        move |index| {
            let ui = ui_handle.unwrap();
            let mut state = state.borrow_mut();
            let idx = index as usize;
            if idx >= state.visible_nodes.len() {
                return;
            }

            state.selected_tree = Some(idx);
            let clicked = state.visible_nodes[idx].clone();

            if clicked.is_folder {
                if !state.expanded_paths.insert(clicked.path.clone()) {
                    state.expanded_paths.remove(&clicked.path);
                }
                refresh_tree(&ui, &mut state);
            } else {
                ui.set_status_text(
                    format!("Selected file: {}", clicked.path.to_string_lossy()).into(),
                );
            }
        }
    });

    ui.on_tab_selected({
        let ui_handle = ui.as_weak();
        let state = state.clone();
        move |tab| {
            let ui = ui_handle.unwrap();
            let mut state = state.borrow_mut();
            state.active_tab = tab;
            ui.set_active_tab(tab);
            state.selected_tree = None;
            refresh_tree(&ui, &mut state);
        }
    });

    ui.on_search({
        let ui_handle = ui.as_weak();
        let state = state.clone();
        move |query| {
            let ui = ui_handle.unwrap();
            let mut state = state.borrow_mut();
            state.selected_tree = None;
            run_search(&ui, &mut state, query.as_str());
        }
    });

    ui.run()?;
    Ok(())
}
