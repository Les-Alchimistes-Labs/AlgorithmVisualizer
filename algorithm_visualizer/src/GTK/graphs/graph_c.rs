use gtk::prelude::*;
use gtk::{Grid, Paned ,Orientation, ComboBoxText, Button, Notebook, Entry, Label
	, Window, MessageDialog, DialogFlags, MessageType, ButtonsType,Image  };

use std::cell::RefCell;
use std::process::Command;

use gdk_pixbuf::Pixbuf;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::env;
