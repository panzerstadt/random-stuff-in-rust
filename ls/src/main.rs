mod basic;

use std::path::Path;

use basic::basic_ls;

/**
 * ref: https://linux.die.net/man/1/ls
 * macOS ref: https://web.archive.org/web/20170909164539/https://developer.apple.com/legacy/library/documentation/Darwin/Reference/ManPages/man1/ls.1.html
 *
 * expected output for ls -l
total 0
drwxr-xr-x  7 tliqun  staff  224 Oct 19 20:46 echo
drwxr-xr-x  4 tliqun  staff  128 Oct 23 20:49 ls
drwxr-xr-x  7 tliqun  staff  224 Oct 19 20:41 yes

broken down:
- file permissions for user, group, others
- num of dir entries if dir, num of hardlinks if file
- owner name
- group name
- number of bytes in the file
- date when the file was last modified
- path name
 */
fn main() {
    basic_ls(Path::new("."));
}
