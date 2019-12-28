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
#[derive(Clone, Debug)]
pub struct Chroot {
    skip_chdir: bool,
    user_spec: Option<OsString>,
    groups: Option<OsString>,
}

impl Default for Chroot {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Chroot {
    /// Creates an instance suitable for setting up a `Command` to execute a
    /// program through `chroot`.
    #[inline]
    pub const fn new() -> Self {
        Self {
            skip_chdir: false,
            user_spec: None,
            groups: None,
        }
    }

    /// Do not change the working directory to `/`.
    #[inline]
    pub fn skip_chdir(mut self) -> Self {
        self.skip_chdir = true;
        self
    }

    /// Specify the user to use.
    pub fn user<U>(mut self, user: U) -> Self
    where
        U: AsRef<OsStr>,
    {
        let mut user_spec = OsString::from("--userspec=");
        user_spec.push(user);
        self.user_spec = Some(user_spec);
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
        self.user_spec = Some(user_spec);
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

            self.groups = Some(arg);
        }

        self
    }

    // Monomorphized form of `command` to reduce binary size.
    fn command_impl(&self, root: &OsStr, program: &OsStr) -> Command {
        let mut command = Command::new("chroot");

        if self.skip_chdir {
            command.arg("--skip-chdir");
        }

        if let Some(user_spec) = &self.user_spec {
            command.arg(user_spec);
        }

        if let Some(groups) = &self.groups {
            command.arg(groups);
        }

        command.arg(root);
        command.arg(program);

        command
    }

    /// Returns a `Command` suitable for spawning `program` with `root` as `/`.
    #[inline]
    pub fn command<R, P>(&self, root: R, program: P) -> Command
    where
        R: AsRef<Path>,
        P: AsRef<OsStr>,
    {
        self.command_impl(root.as_ref().as_os_str(), program.as_ref())
    }
}
