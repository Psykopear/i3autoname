#[macro_use]
extern crate lazy_static;

use i3ipc::event::{inner::WindowChange, Event};
use i3ipc::reply::{Node, NodeType, WindowProperty};
use i3ipc::Subscription;
use i3ipc::{I3Connection, I3EventListener};

mod icons;
use icons::ICONS;

fn find_leaves(node: &Node, leaves: &mut Vec<Node>) {
    // nodes.len == 0 means it's a leave.
    // node.name.is_some() means it's an application
    if node.nodes.is_empty()
        && node.name.is_some()
        && (node.nodetype == NodeType::Con || node.nodetype == NodeType::FloatingCon)
    {
        if let Some(props) = &node.window_properties {
            if props.get(&WindowProperty::Class).is_some() {
                leaves.push(node.clone());
            }
        }
    } else {
        for node in node.nodes.iter().chain(node.floating_nodes.iter()) {
            find_leaves(node, leaves);
        }
    }
}

fn find_workspaces(node: &Node, workspaces: &mut Vec<Node>) {
    if node.nodetype == NodeType::Workspace {
        if node.name.is_some() {
            workspaces.push(node.clone());
        }
    } else {
        for node in &node.nodes {
            find_workspaces(node, workspaces);
        }
    }
}

fn rename_workspaces() {
    let mut connection = I3Connection::connect().unwrap();
    let ws = connection.get_workspaces().unwrap();
    for output in connection
        .get_tree()
        .unwrap()
        .nodes
        .iter()
        .filter(|n| n.nodetype == NodeType::Output)
    {
        let mut workspaces = vec![];
        find_workspaces(&output, &mut workspaces);
        for workspace in workspaces {
            let mut leaves = vec![];
            find_leaves(&workspace, &mut leaves);
            if let Some(_ws) = ws
                .workspaces
                .iter()
                .find(|w| &w.name == workspace.name.as_ref().unwrap())
            {
                let apps: Vec<String> = leaves
                    .iter()
                    .map(|n| {
                        n.window_properties
                            .as_ref()
                            .unwrap()
                            .get(&WindowProperty::Class)
                            .unwrap()
                            .to_string()
                    })
                    .collect();
                let icons: Vec<String> = apps
                    .iter()
                    .map(|a| match ICONS.get(&a.to_lowercase()) {
                        Some(icon) => icon.to_string(),
                        None => " * ".to_string(),
                    })
                    .collect();
                let new_name = format!("{}  {}", _ws.num, icons.join(" "));
                let command = format!(
                    "rename workspace \"{}\" to \"{}\"",
                    workspace.name.unwrap(),
                    new_name
                );
                connection
                    .run_command(&command)
                    .expect("failed to send command");
            }
        }
    }
}

fn main() {
    // First set the correct names for the first time
    rename_workspaces();
    // Then listen to window events and update names when necessary
    let mut listener = I3EventListener::connect().unwrap();
    let subs = [Subscription::Window];
    listener.subscribe(&subs).unwrap();

    for event in listener.listen() {
        match event.unwrap() {
            Event::WindowEvent(e) => match e.change {
                WindowChange::New => {
                    rename_workspaces();
                }
                WindowChange::Close => {
                    rename_workspaces();
                }
                WindowChange::Move => {
                    rename_workspaces();
                }
                _ => (),
            },
            _ => unreachable!(),
        }
    }
}
