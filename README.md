# leetlocal

Scaffolding to bring leetcode questions local, so you can work on problems in your local IDE.

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
