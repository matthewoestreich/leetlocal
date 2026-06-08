# leetlocal

Scaffolding to bring leetcode questions local, so you can work on problems in your local IDE.

# Usage

```bash
leetlocal --title "partition-array-according-to-given-pivot" --language JavaScript --output-dir .
```

⚠️**IMPORTANT** ⚠️

The `--title` argument must match the title slug used in the URL!

For example:

- If the URL is `https://leetcode.com/problems/two-sum/`, your command will look like:

```bash
leetlocal --title "two-sum" --language <language> --output-dir <path_to_dir>
```

- If the URL is `https://leetcode.com/problems/partition-array-according-to-given-pivot/`, your command will look like:

```bash
leetlocal --title "partition-array-according-to-given-pivot" --language <language> --output-dir <path_to_dir>
```
