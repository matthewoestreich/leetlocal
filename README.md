# leetlocal

Scaffolding to bring leetcode questions local, so you can work on problems in your local IDE.used

```
Scaffolding to bring leetcode questions local

Usage: leetlocal [OPTIONS] --problem <PROBLEM> --language <LANGUAGE> --output-dir <OUTPUT_DIR>

Options:
  -p, --problem <PROBLEM>
          Query leetcode for question based upon it's title slug
  -l, --language <LANGUAGE>
          The language you want to solve the problem in [possible values: Cpp, Java, Python3, Python, JavaScript, TypeScript, CSharp, C, Go, Kotlin, Swift, Rust, Ruby, Php, Dart, Scala, Elixir, Erlang, Racket, Cangjie, Bash, React, MySQL, MSSQL, PostgreSQL, OracleSQL, Pandas]
  -o, --output-dir <OUTPUT_DIR>
          Destination directory where the question files will be created - **must be a directory** - path is relative to current working directory (where the CLI is being called from)
  -d, --description-file-name <DESCRIPTION_FILE_NAME>
          The file name for the problem description, saved as markdown, so no file extension needed [default: README]
  -c, --code-file-name <CODE_FILE_NAME>
          The file name for the problem code. Automatically saved with appropriate file extension based on '--language', so no need to provide an extension [default: main]
  -f, --force
          Force creation of output directory and/or overwrite existing files
  -h, --help
          Print help
  -V, --version
          Print version
```

# Usage

```bash
leetlocal --problem "partition-array-according-to-given-pivot" --language JavaScript --output-dir .
```

⚠️ **IMPORTANT** ⚠️

The `--problem` argument must match the title slug used in the URL!

For example:

- If the URL is `https://leetcode.com/problems/two-sum/`, your command will look like:

```bash
leetlocal --problem "two-sum" --language <language> --output-dir <path_to_dir>
```

- If the URL is `https://leetcode.com/problems/partition-array-according-to-given-pivot/`, your command will look like:

```bash
leetlocal --problem "partition-array-according-to-given-pivot" --language <language> --output-dir <path_to_dir>
```

**TODO**

- Automatically scaffold language agnostic test cases
