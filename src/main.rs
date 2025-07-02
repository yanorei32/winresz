use clap::Parser;
use winsafe::{
    co::{PROCESS, PROCESS_NAME, SWP},
    EnumWindows, HwndPlace, HPROCESS, HWND, POINT,
};

mod model;
use model::*;

fn filter_target_windows(hwnd: &HWND, q: &TargetInformation) -> bool {
    if !q.title_contains.is_empty() {
        let Ok(title) = hwnd.GetWindowText() else {
            return false;
        };

        if q.title_contains
            .iter()
            .all(|s| !title.to_ascii_lowercase().contains(&s.to_ascii_lowercase()))
        {
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

        if q.path_endswith
            .iter()
            .all(|s| !path.to_ascii_lowercase().ends_with(&s.to_ascii_lowercase()))
        {
            return false;
        }
    }

    true
}

fn window_callback(hwnd: HWND, op: &Cli) {
    if !filter_target_windows(&hwnd, &op.target) {
        return;
    }

    let client_size = Size::from(hwnd.GetClientRect().unwrap());

    let Some(size) = op.size else {
        println!("{}\t", client_size - op.target.offset);
        return;
    };

    let window_size = Size::from(hwnd.GetWindowRect().unwrap());

    let border = window_size - client_size;

    hwnd.SetWindowPos(
        HwndPlace::None,
        POINT::with(0, 0),
        (size + op.target.offset + border).into(),
        SWP::NOMOVE,
    )
    .unwrap();
}

fn main() {
    let c = Cli::parse();
    EnumWindows(|hwnd: HWND| -> bool {
        window_callback(hwnd, &c);
        true
    })
    .unwrap();
}
