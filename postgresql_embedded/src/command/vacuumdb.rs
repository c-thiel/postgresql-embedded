use crate::command::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// vacuumdb cleans and analyzes a PostgreSQL database.
#[derive(Clone, Debug, Default)]
pub struct VacuumDbBuilder {
    program_dir: Option<PathBuf>,
    all: bool,
    buffer_usage_limit: Option<OsString>,
    dbname: Option<OsString>,
    disable_page_skipping: bool,
    echo: bool,
    full: bool,
    freeze: bool,
    force_index_cleanup: bool,
    jobs: Option<u32>,
    min_mxid_age: Option<OsString>,
    min_xid_age: Option<OsString>,
    no_index_cleanup: bool,
    no_process_main: bool,
    no_process_toast: bool,
    no_truncate: bool,
    schema: Option<OsString>,
    exclude_schema: Option<OsString>,
    parallel: Option<u32>,
    quiet: bool,
    skip_locked: bool,
    table: Option<OsString>,
    verbose: bool,
    version: bool,
    analyze: bool,
    analyze_only: bool,
    analyze_in_stages: bool,
    help: bool,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    maintenance_db: Option<OsString>,
}

/// vacuumdb cleans and analyzes a PostgreSQL database.
impl VacuumDbBuilder {
    /// Create a new [`VacuumDbBuilder`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// vacuum all databases
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// size of ring buffer used for vacuum
    pub fn buffer_usage_limit<S: AsRef<OsStr>>(mut self, buffer_usage_limit: S) -> Self {
        self.buffer_usage_limit = Some(buffer_usage_limit.as_ref().to_os_string());
        self
    }

    /// database to vacuum
    pub fn dbname<S: AsRef<OsStr>>(mut self, dbname: S) -> Self {
        self.dbname = Some(dbname.as_ref().to_os_string());
        self
    }

    /// disable all page-skipping behavior
    pub fn disable_page_skipping(mut self) -> Self {
        self.disable_page_skipping = true;
        self
    }

    /// show the commands being sent to the server
    pub fn echo(mut self) -> Self {
        self.echo = true;
        self
    }

    /// do full vacuuming
    pub fn full(mut self) -> Self {
        self.full = true;
        self
    }

    /// freeze row transaction information
    pub fn freeze(mut self) -> Self {
        self.freeze = true;
        self
    }

    /// always remove index entries that point to dead tuples
    pub fn force_index_cleanup(mut self) -> Self {
        self.force_index_cleanup = true;
        self
    }

    /// use this many concurrent connections to vacuum
    pub fn jobs(mut self, jobs: u32) -> Self {
        self.jobs = Some(jobs);
        self
    }

    /// minimum multixact ID age of tables to vacuum
    pub fn min_mxid_age<S: AsRef<OsStr>>(mut self, min_mxid_age: S) -> Self {
        self.min_mxid_age = Some(min_mxid_age.as_ref().to_os_string());
        self
    }

    /// minimum transaction ID age of tables to vacuum
    pub fn min_xid_age<S: AsRef<OsStr>>(mut self, min_xid_age: S) -> Self {
        self.min_xid_age = Some(min_xid_age.as_ref().to_os_string());
        self
    }

    /// don't remove index entries that point to dead tuples
    pub fn no_index_cleanup(mut self) -> Self {
        self.no_index_cleanup = true;
        self
    }

    /// skip the main relation
    pub fn no_process_main(mut self) -> Self {
        self.no_process_main = true;
        self
    }

    /// skip the TOAST table associated with the table to vacuum
    pub fn no_process_toast(mut self) -> Self {
        self.no_process_toast = true;
        self
    }

    /// don't truncate empty pages at the end of the table
    pub fn no_truncate(mut self) -> Self {
        self.no_truncate = true;
        self
    }

    /// vacuum tables in the specified schema(s) only
    pub fn schema<S: AsRef<OsStr>>(mut self, schema: S) -> Self {
        self.schema = Some(schema.as_ref().to_os_string());
        self
    }

    /// do not vacuum tables in the specified schema(s)
    pub fn exclude_schema<S: AsRef<OsStr>>(mut self, exclude_schema: S) -> Self {
        self.exclude_schema = Some(exclude_schema.as_ref().to_os_string());
        self
    }

    /// use this many background workers for vacuum, if available
    pub fn parallel(mut self, parallel: u32) -> Self {
        self.parallel = Some(parallel);
        self
    }

    /// don't write any messages
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// skip relations that cannot be immediately locked
    pub fn skip_locked(mut self) -> Self {
        self.skip_locked = true;
        self
    }

    /// vacuum specific table(s) only
    pub fn table<S: AsRef<OsStr>>(mut self, table: S) -> Self {
        self.table = Some(table.as_ref().to_os_string());
        self
    }

    /// write a lot of output
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// output version information, then exit
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// update optimizer statistics
    pub fn analyze(mut self) -> Self {
        self.analyze = true;
        self
    }

    /// only update optimizer statistics; no vacuum
    pub fn analyze_only(mut self) -> Self {
        self.analyze_only = true;
        self
    }

    /// only update optimizer statistics, in multiple stages for faster results; no vacuum
    pub fn analyze_in_stages(mut self) -> Self {
        self.analyze_in_stages = true;
        self
    }

    /// show help, then exit
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }

    /// database server host or socket directory
    pub fn host<S: AsRef<OsStr>>(mut self, host: S) -> Self {
        self.host = Some(host.as_ref().to_os_string());
        self
    }

    /// database server port
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// user name to connect as
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }

    /// never prompt for password
    pub fn no_password(mut self) -> Self {
        self.no_password = true;
        self
    }

    /// force password prompt
    pub fn password(mut self) -> Self {
        self.password = true;
        self
    }

    /// alternate maintenance database
    pub fn maintenance_db<S: AsRef<OsStr>>(mut self, maintenance_db: S) -> Self {
        self.maintenance_db = Some(maintenance_db.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for VacuumDbBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "vacuumdb".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if self.all {
            args.push("--all".into());
        }

        if let Some(buffer_usage_limit) = &self.buffer_usage_limit {
            args.push("--buffer-usage-limit".into());
            args.push(buffer_usage_limit.into());
        }

        if let Some(dbname) = &self.dbname {
            args.push("--dbname".into());
            args.push(dbname.into());
        }

        if self.disable_page_skipping {
            args.push("--disable-page-skipping".into());
        }

        if self.echo {
            args.push("--echo".into());
        }

        if self.full {
            args.push("--full".into());
        }

        if self.freeze {
            args.push("--freeze".into());
        }

        if self.force_index_cleanup {
            args.push("--force-index-cleanup".into());
        }

        if let Some(jobs) = &self.jobs {
            args.push("--jobs".into());
            args.push(jobs.to_string().into());
        }

        if let Some(min_mxid_age) = &self.min_mxid_age {
            args.push("--min-mxid-age".into());
            args.push(min_mxid_age.into());
        }

        if let Some(min_xid_age) = &self.min_xid_age {
            args.push("--min-xid-age".into());
            args.push(min_xid_age.into());
        }

        if self.no_index_cleanup {
            args.push("--no-index-cleanup".into());
        }

        if self.no_process_main {
            args.push("--no-process-main".into());
        }

        if self.no_process_toast {
            args.push("--no-process-toast".into());
        }

        if self.no_truncate {
            args.push("--no-truncate".into());
        }

        if let Some(schema) = &self.schema {
            args.push("--schema".into());
            args.push(schema.into());
        }

        if let Some(exclude_schema) = &self.exclude_schema {
            args.push("--exclude-schema".into());
            args.push(exclude_schema.into());
        }

        if let Some(parallel) = &self.parallel {
            args.push("--parallel".into());
            args.push(parallel.to_string().into());
        }

        if self.quiet {
            args.push("--quiet".into());
        }

        if self.skip_locked {
            args.push("--skip-locked".into());
        }

        if let Some(table) = &self.table {
            args.push("--table".into());
            args.push(table.into());
        }

        if self.verbose {
            args.push("--verbose".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.analyze {
            args.push("--analyze".into());
        }

        if self.analyze_only {
            args.push("--analyze-only".into());
        }

        if self.analyze_in_stages {
            args.push("--analyze-in-stages".into());
        }

        if self.help {
            args.push("--help".into());
        }

        if let Some(host) = &self.host {
            args.push("--host".into());
            args.push(host.into());
        }

        if let Some(port) = &self.port {
            args.push("--port".into());
            args.push(port.to_string().into());
        }

        if let Some(username) = &self.username {
            args.push("--username".into());
            args.push(username.into());
        }

        if self.no_password {
            args.push("--no-password".into());
        }

        if self.password {
            args.push("--password".into());
        }

        if let Some(maintenance_db) = &self.maintenance_db {
            args.push("--maintenance-db".into());
            args.push(maintenance_db.into());
        }

        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::traits::CommandToString;

    #[test]
    fn test_builder_new() {
        let command = VacuumDbBuilder::new().program_dir(".").build();

        assert_eq!(
            PathBuf::from(".").join("vacuumdb"),
            PathBuf::from(command.to_command_string().replace("\"", ""))
        );
    }

    #[test]
    fn test_builder() {
        let command = VacuumDbBuilder::new()
            .all()
            .buffer_usage_limit("buffer_usage_limit")
            .dbname("dbname")
            .disable_page_skipping()
            .echo()
            .full()
            .freeze()
            .force_index_cleanup()
            .jobs(1)
            .min_mxid_age("min_mxid_age")
            .min_xid_age("min_xid_age")
            .no_index_cleanup()
            .no_process_main()
            .no_process_toast()
            .no_truncate()
            .schema("schema")
            .exclude_schema("exclude_schema")
            .parallel(1)
            .quiet()
            .skip_locked()
            .table("table")
            .verbose()
            .version()
            .analyze()
            .analyze_only()
            .analyze_in_stages()
            .help()
            .host("localhost")
            .port(5432)
            .username("username")
            .no_password()
            .password()
            .maintenance_db("maintenance_db")
            .build();

        assert_eq!(
            r#""vacuumdb" "--all" "--buffer-usage-limit" "buffer_usage_limit" "--dbname" "dbname" "--disable-page-skipping" "--echo" "--full" "--freeze" "--force-index-cleanup" "--jobs" "1" "--min-mxid-age" "min_mxid_age" "--min-xid-age" "min_xid_age" "--no-index-cleanup" "--no-process-main" "--no-process-toast" "--no-truncate" "--schema" "schema" "--exclude-schema" "exclude_schema" "--parallel" "1" "--quiet" "--skip-locked" "--table" "table" "--verbose" "--version" "--analyze" "--analyze-only" "--analyze-in-stages" "--help" "--host" "localhost" "--port" "5432" "--username" "username" "--no-password" "--password" "--maintenance-db" "maintenance_db""#,
            command.to_command_string()
        );
    }
}
