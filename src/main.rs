use clap::Parser;
use winsafe::{
    co::{PROCESS, PROCESS_NAME, SWP},
    prelude::*,
    EnumWindows, HwndPlace, HPROCESS, HWND, POINT,
};

mod model;
use model::*;

fn filter_target_windows(hwnd: &HWND, q: &TargetInformation) -> bool {
    if !q.title_contains.is_empty() {
        let Ok(title) = hwnd.GetWindowText() else {
            return false;
        };

        if q.title_contains.iter().all(|s| !title.contains(s)) {
            return false;
        }
    }

    if !q.path_endswith.is_empty() {
        let (_, pid) = hwnd.GetWindowThreadProcessId();

        let Ok(proc) = HPROCESS::OpenProcess(PROCESS::QUERY_LIMITED_INFORMATION, false, pid) else {
            return false;
        };

        let Ok(path) = proc.QueryFullProcessImageName(PROCESS_NAME::WIN32) else {
            return false;
        };

        if q.path_endswith.iter().all(|s| !path.ends_with(s)) {
            return false;
        }
    }

    true
}

fn window_callback(hwnd: HWND, op: &Commands) {
    let t = match op {
        Commands::Get { target } => target,
        Commands::Set {
            target,
            resolution: _,
        } => target,
    };

    if !filter_target_windows(&hwnd, t) {
        return;
    }

    let client_size = Size::from(hwnd.GetClientRect().unwrap());

    match op {
        Commands::Get { target: _ } => {
            println!("{}", client_size - t.offset);
        }
        Commands::Set {
            target: _,
            resolution,
        } => {
            let window_size = Size::from(hwnd.GetWindowRect().unwrap());
            let border = window_size - client_size;

            hwnd.SetWindowPos(
                HwndPlace::None,
                POINT::new(0, 0),
                (*resolution + t.offset + border).into(),
                SWP::NOMOVE,
            )
            .unwrap();
        }
    }
}

fn main() {
    let c = Cli::parse().command;
    EnumWindows(|hwnd: HWND| -> bool {
        window_callback(hwnd, &c);
        true
    })
    .unwrap();
}
