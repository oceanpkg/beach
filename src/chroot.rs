use std::{
    ffi::{OsStr, OsString},
    path::Path,
    process::Command,
};

/// A wrapper for [`chroot(1)`](https://www.gnu.org/software/coreutils/chroot).
///
/// **Note:** Running `chroot` requires root privileges.
///
/// # Examples
///
/// ```
/// # return;
/// beach::Chroot::new()
///     .user_group("nvzqz", "everyone")
///     .command("/path/to/root", "ls")
///     .arg("/")
///     .spawn();
/// ```
#[derive(Debug)]
pub struct Chroot(Command);

impl Chroot {
    /// Creates an instance suitable for setting up a `Command` to execute a
    /// program through `chroot`.
    #[inline]
    pub fn new() -> Self {
        Self(Command::new("chroot"))
    }

    /// Do not change the working directory to `/`.
    #[inline]
    pub fn skip_chdir(mut self) -> Self {
        self.0.arg("--skip-chdir");
        self
    }

    /// Specify the user to use.
    pub fn user<U>(mut self, user: U) -> Self
    where
        U: AsRef<OsStr>,
    {
        let mut user_spec = OsString::from("--userspec=");
        user_spec.push(user);
        self.0.arg(user_spec);
        self
    }

    /// Specify the user and group (ID or name) to use.
    pub fn user_group<U, G>(mut self, user: U, group: G) -> Self
    where
        U: AsRef<OsStr>,
        G: AsRef<OsStr>,
    {
        let mut user_spec = OsString::from("--userspec=");
        user_spec.push(user);
        user_spec.push(":");
        user_spec.push(group);
        self.0.arg(user_spec);
        self
    }

    /// Specifies supplementary groups.
    pub fn groups<G>(mut self, groups: G) -> Self
    where
        G: IntoIterator,
        G::Item: AsRef<OsStr>,
    {
        let mut groups = groups.into_iter();

        if let Some(group) = groups.next() {
            // Only add the `groups` argument if the iterator is non-empty.
            let mut arg = OsString::from("--groups=");
            arg.push(group);

            // Add remaining groups as a comma-separated list.
            for group in groups {
                arg.push(",");
                arg.push(group);
            }

            self.0.arg(arg);
        }

        self
    }

    /// Returns a `Command` suitable for spawning `program` with `root` as `/`.
    #[inline]
    pub fn command<R, P>(mut self, root: R, program: P) -> Command
    where
        R: AsRef<Path>,
        P: AsRef<OsStr>,
    {
        self.0.arg(root.as_ref());
        self.0.arg(program);
        self.0
    }
}
