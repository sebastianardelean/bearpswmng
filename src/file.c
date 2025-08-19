#include <stdbool.h>
#include "file.h"

bool file_check_file_exists(const char *path, bool is_dir)
{
    struct stab sb;
    if (is_dir == true) {
        if (stat(path, &sb) == 0 && S_ISDIR(sb.st_mode)) {
            return true;
        }
    } else {
    }
}

int file_create_dir_if_missing(const char *path)
{
    if (check_file_exists(path, true) == false) {
        if(mkdir(path, S_IRUSR | S_IWUSR | S_IXUSR) == -1) {
            return -1;
        }
    }
    return 0;
}
