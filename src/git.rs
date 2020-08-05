use crate::output::{CHECKOUT_EMOJI, DOWNLOAD_EMOJI, INDEX_EMOJI};
use anyhow::Result;
use git2::{
    build::{CheckoutBuilder, RepoBuilder},
    FetchOptions, RemoteCallbacks,
};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::path::Path;

/// Parses the input and creates a valid git repository.
///
/// Currently only github is supported.
pub fn parse_to_git_url(value: &str) -> String {
    let splitted_at_slash_fragments = value.split('/').collect::<Vec<&str>>();
    match splitted_at_slash_fragments.len() {
        2 => format!(
            "https://github.com/{}/{}.git",
            splitted_at_slash_fragments[0], splitted_at_slash_fragments[1]
        ),
        _ => value.to_string(),
    }
}

/// Extracts the repository name from a git url.
///
/// # Note
///
/// This method does not do any safety checks and assumes that the passed url is
/// a valid git url.
pub fn get_repo_name(url: &str) -> &str {
    url.rsplitn(2, '/').next().unwrap().trim_end_matches(".git")
}

// [1/4] Cloning template repository...
// [1/?] ⬇ Downloading...
// [1/?] 🔍 Indexing...
// [1/?] 🚧 Checking out...
// [2/4] ✏ Asking needed questions...
// [3/4] 🔧 Applying features...
// [4/4] 🏗 Filling templates...
// Done

/// Clones a repository into the given folder.
pub fn clone_into_folder(
    url: &str,
    branch: &Option<impl AsRef<str>>,
    folder_path: impl AsRef<Path>,
) -> Result<()> {
    // let url = url::Url::parse(url).unwrap();
    // let ps = ProgressStyle::default_bar()
    //     .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
    //     .template("{prefix:.bold.dim} {spinner} {wide_msg}");

    // Default message style.
    let ps =
        ProgressStyle::default_bar().template("{prefix:.bold.dim} {msg} {wide_bar} {pos}/{len}");

    let mb = MultiProgress::new();
    let network_pb = mb.add(ProgressBar::new(0));
    let indexed_pb = mb.add(ProgressBar::new(0));
    let checkout_pb = mb.add(ProgressBar::new(0));

    network_pb.set_style(ps.clone());
    network_pb.set_prefix("[1/?]");
    network_pb.set_message(&format!("{} {:<15}", DOWNLOAD_EMOJI, "Downloading..."));
    indexed_pb.set_style(ps.clone());
    indexed_pb.set_prefix("[2/?]");
    indexed_pb.set_message(&format!("{} {:<15}", INDEX_EMOJI, "Indexing..."));
    checkout_pb.set_style(ps);
    checkout_pb.set_prefix("[3/?]");
    checkout_pb.set_message(&format!("{} {:<15}", CHECKOUT_EMOJI, "Checking out..."));

    let mut cb = RemoteCallbacks::new();
    cb.transfer_progress(move |stats| {
        network_pb.set_length(stats.total_objects() as u64);
        network_pb.set_position(stats.received_objects() as u64);

        indexed_pb.set_length(stats.total_objects() as u64);
        indexed_pb.set_position(stats.indexed_objects() as u64);

        if stats.received_objects() == stats.total_objects() {
            network_pb.finish_with_message(&format!("{} Done", DOWNLOAD_EMOJI));
        }
        if stats.indexed_objects() == stats.total_objects() {
            indexed_pb.finish_with_message(&format!("{} Done", INDEX_EMOJI));
        }

        true
    });

    let mut co = CheckoutBuilder::new();

    co.progress(move |_path, cur, total| {
        checkout_pb.set_length(total as u64);
        checkout_pb.set_position(cur as u64);

        if cur == total {
            checkout_pb.finish_with_message(&format!("{} Done", CHECKOUT_EMOJI));
        }
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);

    let handle = std::thread::spawn(move || {
        mb.join().unwrap();
        mb.join_and_clear()
            .expect("failed to join and wait on progressbars");
    });

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo).with_checkout(co);
    if let Some(branch_name) = branch {
        builder.branch(branch_name.as_ref());
    }
    builder.clone(url, folder_path.as_ref())?;

    // Wait for the progress bar thread to actually finish. This prevents weird
    // clear and print fragments.
    handle
        .join()
        .expect("failed to join no progress bar thread");

    Ok(())
}
