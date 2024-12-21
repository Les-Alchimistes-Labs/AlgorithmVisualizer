use gtk::prelude::*;
use crate::GTK::file::*;
use crate::GTK::edit::*;
use crate::GTK::help::*;


pub fn create_menu_bar() -> gtk::MenuBar 
{
    let menu_bar = gtk::MenuBar::new();
    
    let file_menu = file_menu_create();
    let edition_menu = edition_menu_create();
    let help_menu = help_menu_create();

    menu_bar.append(&file_menu);
    menu_bar.append(&edition_menu);
    menu_bar.append(&help_menu);

    menu_bar
}

fn file_menu_create() -> gtk::MenuItem 
{
    let result = gtk::MenuItem::with_label("File");
    let open = gtk::MenuItem::with_label("Open");
    let sub_open = gtk::Menu::new();
    let list_open = gtk::MenuItem::with_label("list");
    let tree_open = gtk::MenuItem::with_label("tree");
    let graph_open = gtk::MenuItem::with_label("graph");
    let item_quit = gtk::MenuItem::with_label("Quit");
    
    let with_cost = gtk::MenuItem::with_label("undirected with cost");
    let with_cost1 = gtk::MenuItem::with_label("directed with cost");
    let without_cost = gtk::MenuItem::with_label("undirected without cost");
    let without_cost1 = gtk::MenuItem::with_label("directed without cost");
    let sub_graph = gtk::Menu::new();

    graph_open.set_submenu(Some(&sub_graph));
    sub_graph.append(&with_cost);
    sub_graph.append(&without_cost);
    sub_graph.append(&with_cost1);
    sub_graph.append(&without_cost1);
    let menu_items = gtk::Menu::new();

    sub_open.append(&list_open);
    sub_open.append(&tree_open);
    sub_open.append(&graph_open);
    menu_items.append(&open);
    menu_items.append(&item_quit);
    open.set_submenu(Some(&sub_open));
    
    result.set_submenu(Some(&menu_items));
    
    list_open.connect_activate(|_| {
        open_list();
    });
    tree_open.connect_activate(|_| {
        open_dot(0);
    });
    with_cost1.connect_activate(|_| {
        open_dot(1);
    });
    with_cost.connect_activate(|_| {
        open_dot(3);
    });
    without_cost.connect_activate(|_| {
        open_dot(4);
    });
    without_cost1.connect_activate(|_| {
        open_dot(2);
    });
      
    item_quit.connect_activate(|_| gtk::main_quit());
    result
}

fn edition_menu_create() -> gtk::MenuItem {
    let result = gtk::MenuItem::with_label("Edit");
    
    let as_image = gtk::MenuItem::with_label("Save as image");
    let menu_img = gtk::Menu::new();
    let img_tree = gtk::MenuItem::with_label("Tree");
    let img_graph = gtk::MenuItem::with_label("Graph");
    
    menu_img.append(&img_tree);
    menu_img.append(&img_graph);
    as_image.set_submenu(Some(&menu_img));
    
    let as_text = gtk::MenuItem::with_label("Save as text");
    let menu_txt = gtk::Menu::new();
    let txt_list = gtk::MenuItem::with_label("List");
    let txt_tree = gtk::MenuItem::with_label("Tree");
    let txt_graph = gtk::MenuItem::with_label("Graph");
    
    let with_cost = gtk::MenuItem::with_label("undirected with cost");
    let with_cost1 = gtk::MenuItem::with_label("directed with cost");
    let without_cost = gtk::MenuItem::with_label("undirected without cost");
    let without_cost1 = gtk::MenuItem::with_label("directed without cost");
    let sub_graph = gtk::Menu::new();

    img_graph.set_submenu(Some(&sub_graph));
    sub_graph.append(&with_cost);
    sub_graph.append(&without_cost);
    sub_graph.append(&with_cost1);
    sub_graph.append(&without_cost1);
    
    let with_cost2 = gtk::MenuItem::with_label("undirected with cost");
    let with_cost12 = gtk::MenuItem::with_label("directed with cost");
    let without_cost2 = gtk::MenuItem::with_label("undirected without cost");
    let without_cost12 = gtk::MenuItem::with_label("directed without cost");
    let sub_graph2 = gtk::Menu::new();

    txt_graph.set_submenu(Some(&sub_graph2));
    sub_graph2.append(&with_cost2);
    sub_graph2.append(&without_cost2);
    sub_graph2.append(&with_cost12);
    sub_graph2.append(&without_cost12);
    
    menu_txt.append(&txt_list);
    menu_txt.append(&txt_tree);
    menu_txt.append(&txt_graph);
    as_text.set_submenu(Some(&menu_txt));
    
    let menu_items = gtk::Menu::new();

    menu_items.append(&as_image);
    menu_items.append(&as_text);

    result.set_submenu(Some(&menu_items));
    
    txt_list.connect_activate(|_| {
       save_list_text();
    });
    
    txt_tree.connect_activate(|_| {
       save_dot("tree");
    });
    
    img_tree.connect_activate(|_| {
       save_png("tree");
    });
    
    with_cost.connect_activate(|_| {
       save_png("ucgraph");
    });
    without_cost.connect_activate(|_| {
       save_png("ugraph");
    });
    with_cost1.connect_activate(|_| {
       save_png("dicgraph");
    });
    without_cost1.connect_activate(|_| {
       save_png("digraph");
    });
    with_cost2.connect_activate(|_| {
       save_dot("ucgraph");
    });
    without_cost2.connect_activate(|_| {
       save_dot("ugraph");
    });
    with_cost12.connect_activate(|_| {
       save_dot("dicgraph");
    });
    without_cost12.connect_activate(|_| {
       save_dot("digraph");
    });

    result
}

fn help_menu_create() -> gtk::MenuItem 
{
    let result = gtk::MenuItem::with_label("Help");
    let item_info = gtk::MenuItem::with_label("Informations");
    let Hlist = gtk::MenuItem::with_label("List");
    let Htree = gtk::MenuItem::with_label("Tree");
    let Hgraph = gtk::MenuItem::with_label("Graph");
    let sub_info = gtk::Menu::new();
    sub_info.append(&Hlist);
    sub_info.append(&Htree);
    sub_info.append(&Hgraph);
    item_info.set_submenu(Some(&sub_info));
    
    let item_credits = gtk::MenuItem::with_label("Credits");
    let menu_items = gtk::Menu::new();
    menu_items.append(&item_info);
    menu_items.append(&item_credits);

    result.set_submenu(Some(&menu_items));

    Hlist.connect_activate(|_| {
       list_info();
    });
    Htree.connect_activate(|_| {
       tree_info();
    });
    Hgraph.connect_activate(|_| {
       graph_info();
    });
    item_credits.connect_activate(|_| {
       credits();
    });
    result
}


