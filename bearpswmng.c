#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <stdbool.h>

#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <dirent.h>
#include <errno.h>
#include <pwd.h>
#include <getopt.h>
#include <fcntl.h>

#define MAX_FILE_NAME_SIZE 256u
#define PASSMNG_DIR_PATH_SIZE 512u

#define MAX_PASSW_SIZE 2048u
#define MAX_PROC_OUTPUT 2048u

#define MAX_COMMAND_SIZE 4096u

#define MAX_RECIPIENT_SIZE 256u
#define PASSMNG_DIR ".bearpswmng"

char passmng_dir[PASSMNG_DIR_PATH_SIZE] = {0};

struct entry_t
{
  size_t index;
  char name[MAX_FILE_NAME_SIZE];
  char group[MAX_FILE_NAME_SIZE];
  char path[PASSMNG_DIR_PATH_SIZE];

};

static const struct option long_options[] = {
  {"list", no_argument, 0, 'l'},
  {"show", required_argument, 0, 's'},
  {"add", required_argument, 0, 'a'},
  {"help", no_argument, 0, 'h'},
  {0,0,0,0} /* Termination */
};

static struct entry_t *listed_entries = NULL;
static size_t listed_entries_size = 0;

static void print_usage(const char *app);
static int read_directory(const char *path);
static int create_dir_if_missing(const char *path);
static bool check_directory_exists(const char *path);
static int add_entry(const char *group,
                      const char *file,
                      const char *password,
                      const char *recipient);
static int show_entry(const size_t entry_index);
static int show_all_entries();
static int create_password_file(const char *file,
                                 const char *password
                                );
static void encrypt_file(const char *infile,
                         const char *outfile,
                         const char *recipient);

static void decrypt_file(const char *infile);

static void cleanup();

int main(int argc, char **argv)
{
  int option;
  struct passwd *pwd = NULL;
  
  
  if (argc == 1) {
    print_usage(argv[0]);
    exit(EXIT_SUCCESS);
  }

  pwd = getpwuid(getuid());
  if (pwd == NULL) {
    cleanup();
    exit(EXIT_FAILURE);
  }

  snprintf(passmng_dir, PASSMNG_DIR_PATH_SIZE, "%s/%s",pwd->pw_dir,PASSMNG_DIR);

  /* Create the directory if is missing */
  if (create_dir_if_missing(passmng_dir) == EXIT_FAILURE) {
    cleanup();
    exit(EXIT_FAILURE);
  }
  read_directory(passmng_dir);  
  while ((option = getopt_long(argc, argv, "ls:a:h", long_options, NULL)) != -1) {
    switch (option) {
    case 'l':
      show_all_entries();
      break;
    case 's':
      printf("Show details of entry: %lu\n", strtoul(optarg, NULL, 10));
      show_entry(strtoul(optarg, NULL, 10));
      break;
    case 'a': {
      if (optind + 2 != argc) {
        printf("Error: --add option requires three additional arguments.\n");
        print_usage(argv[0]);
        cleanup();
        exit(EXIT_FAILURE);
      }
      char domain[MAX_FILE_NAME_SIZE] = {0};
      char group[MAX_FILE_NAME_SIZE] = {0};
      char recipient[MAX_RECIPIENT_SIZE] = {0};
      strncpy(group, optarg, strlen(optarg));
      strncpy(domain, argv[optind], strlen(argv[optind]));
      optind++;
      strncpy(recipient, argv[optind], strlen(argv[optind]));
      optind++;
      char password[MAX_PASSW_SIZE]={0};
      printf("Enter the password for %s/%s:\n",group, domain);
      scanf("%2047s", password);
      if(add_entry(group, domain, password, recipient)==EXIT_FAILURE) {
        cleanup();
        exit(EXIT_FAILURE);
      }
      break;
    }
    case 'h':
      print_usage(argv[0]);
      break;
    case '?':
      // getopt_long already printed an error message
      break;
    default:
      print_usage(argv[0]);
      break;
    }
  }

  
  cleanup();
  exit(EXIT_SUCCESS);
}

void cleanup()
{
  if (listed_entries != NULL) {
    free(listed_entries);
  }
}

void print_usage(const char *app) {
  printf("Usage: %s [options]\n", app);
  printf("Options:\n");
  printf("  --list                   List all entries\n");
  printf("  --show <entry>           Show details of a specific entry\n");
  printf("  --add <Group> <Domain> <Recipient> Add a new entry\n");
  printf("  -h, --help               Print this help message\n");
}

int show_all_entries()
{
  size_t i = 0;
  char last_printed_group[MAX_FILE_NAME_SIZE] = {0};
  if (listed_entries == NULL) {
    return EXIT_FAILURE;
  }

  for (i = 0; i < listed_entries_size; i++) {
    if (strncmp(last_printed_group, listed_entries[i].group,strlen(listed_entries[i].group))!=0) {
      strncpy(last_printed_group, listed_entries[i].group,strlen(listed_entries[i].group));
      printf("%s\n", listed_entries[i].group);
    }
    printf("\t%lu. %s\n", listed_entries[i].index, listed_entries[i].name);
  }

  return EXIT_SUCCESS;
}

int show_entry(const size_t entry_index)
{
  if (listed_entries == NULL) {
    return EXIT_FAILURE;
  }
  if ((entry_index-1) > listed_entries_size) {
    return EXIT_FAILURE;
  }

  struct entry_t entry = listed_entries[entry_index-1];
  decrypt_file(entry.path);
  printf("\n");
  return EXIT_SUCCESS;
}


void decrypt_file(const char *infile)
{//TODO: I don't like fgets
  FILE *fp;
  char proc_output[MAX_PROC_OUTPUT];
  char command[MAX_COMMAND_SIZE] = {0};

  snprintf(command, MAX_COMMAND_SIZE, "gpg -d %s",infile);
    
  fp = popen(command, "r");
  if (fp == NULL) {
    printf("Failed to run command\n");
  }
  
    // Read the output of the command
  while (fgets(proc_output, sizeof(proc_output), fp) != NULL) {
    printf("%s", proc_output);
  }
  
  // Close the pipe
  pclose(fp);
}

void encrypt_file(const char *infile,const char *outfile, const char *recipient)
{//TODO: I don't like fgets
  FILE *fp;
  char proc_output[MAX_PROC_OUTPUT];
  char command[MAX_COMMAND_SIZE] = {0};

  snprintf(command, MAX_COMMAND_SIZE, "gpg -e -r %s --output %s %s", recipient, outfile, infile);
    
  fp = popen(command, "r");
  if (fp == NULL) {
    printf("Failed to run command\n");
  }
  
    // Read the output of the command
  while (fgets(proc_output, sizeof(proc_output), fp) != NULL) {
    printf("%s", proc_output);
  }
  
  // Close the pipe
  pclose(fp);
}


int add_entry(const char *group,
              const char *file,
              const char *password,
              const char *recipient)
{
  char *group_path = malloc((strlen(passmng_dir)+strlen(group)+2) * sizeof(char));
  if (group_path == NULL) {
    fprintf(stderr, "Could not allocate memory for the group %s path", group);
    return EXIT_FAILURE;
  }
  memset(group_path, 0, (strlen(passmng_dir)+strlen(group)+2));
  snprintf(group_path, (strlen(passmng_dir)+strlen(group)+2), "%s/%s",passmng_dir, group);
  
  
  char *file_path = malloc((strlen(group_path)+strlen(file)+2) * sizeof(char));
  if (file_path == NULL) {
    fprintf(stderr, "Could not allocate memory for the file %s path", file);
    free(group_path);
    return EXIT_FAILURE;
  }
  memset(file_path, 0, (strlen(group_path)+strlen(file)+2));
  snprintf(file_path, (strlen(group_path)+strlen(file)+2), "%s/%s",group_path, file);
  
  char *outfile = malloc((strlen(group_path)+strlen(file)+strlen(".gpg")+2) * sizeof(char));
  if (outfile == NULL) {
    fprintf(stderr, "Could not allocate memory for the file %s path", file);
    free(group_path);
    free(file_path);
    return EXIT_FAILURE;
  }
  memset(outfile, 0, (strlen(file_path)+strlen(".gpg")+2));
  snprintf(outfile, (strlen(file_path)+strlen(".gpg")+2), "%s.gpg",file_path);

  /* Create missing directories if needed */
  if (create_dir_if_missing(group_path) == EXIT_FAILURE) {
    free(file_path);
    free(group_path);
    free(outfile);
    return EXIT_FAILURE;
  }

  if (create_password_file(file_path, password) == EXIT_FAILURE) {
    free(file_path);
    free(group_path);
    free(outfile);
    return EXIT_FAILURE;
  }

  /*Encrypt the file and remove it*/
  encrypt_file(file_path,outfile,recipient);
  
  remove(file_path);
  free(file_path);
  free(group_path);
  free(outfile);
  return EXIT_SUCCESS;
}


int create_password_file(const char *file,
                          const char *password)
{
  int fd = -1;

  fd = open(file, O_RDWR | O_CREAT, S_IRUSR|S_IWUSR);
  if (fd == -1) {
    fprintf(stderr, "Can't create password file %s - Error %s\n", file, strerror(errno));
    return EXIT_FAILURE;
  }

  /*Write the password*/
  if (write(fd, password, strlen(password)) == -1) {
    fprintf(stderr, "Can't write to file %s - Error %s\n", file, strerror(errno));
    close(fd);
    return EXIT_FAILURE;
  }
  close(fd);
  return EXIT_SUCCESS;
}

int read_directory(const char *path)
{
  static int file_index = 1;
  DIR *directory = opendir(path);
  char subdir_path[PASSMNG_DIR_PATH_SIZE] = {0};
  if (check_directory_exists(path) == false) {
    fprintf(stderr, "Directory %s does not exist!", path);
    return EXIT_FAILURE;
  }
  
  if (directory == NULL) {
    fprintf(stderr, "Can't open directory %s - Error %s\n", path, strerror(errno));
    return EXIT_FAILURE;
  }

  struct dirent *entry = NULL;

  while ((entry = readdir(directory)) != NULL) {
    snprintf(subdir_path, PASSMNG_DIR_PATH_SIZE, "%s/%s", path, entry->d_name);
    if (entry->d_type == DT_DIR) {
      if (strcmp(entry->d_name, ".") != 0 && strcmp(entry->d_name, "..")!=0) {

        //        printf("%s\n", entry->d_name);
        read_directory(subdir_path);
      }
      
    }
    else {
      struct entry_t *ptr = realloc(listed_entries, file_index*sizeof(struct entry_t));
      if (ptr==NULL) {
        fprintf(stderr, "Could not allocate memory");
        return EXIT_FAILURE;
      }
      listed_entries=ptr;
      listed_entries[file_index-1].index = file_index;
      memset(listed_entries[file_index-1].path, 0, strlen(subdir_path));
      memset(listed_entries[file_index-1].name, 0, strlen(entry->d_name));
      memset(listed_entries[file_index-1].group,0, strlen(subdir_path));
      strncpy(listed_entries[file_index-1].name, entry->d_name, strlen(entry->d_name)+1);
      strncpy(listed_entries[file_index-1].path, subdir_path,strlen(subdir_path)+1);

      /*just a little hack to get the substring*/
      size_t passmng_dir_len = strlen(passmng_dir)+1;
      strncpy(listed_entries[file_index-1].group, subdir_path+passmng_dir_len, strlen(subdir_path+passmng_dir_len));
      listed_entries[file_index-1].group[strlen(listed_entries[file_index-1].group)-strlen(entry->d_name)-1]='\0';
      file_index++;
      listed_entries_size++;
    }
  }

  closedir(directory);
  return EXIT_SUCCESS;
  
}

bool check_directory_exists(const char *path)
{
  struct stat sb;
  //  printf("Checking directory %s",path);
  if (stat(path, &sb) == 0 && S_ISDIR(sb.st_mode)) {
    return true;
  }
  return false;
}

int create_dir_if_missing(const char *path)
{

  if (check_directory_exists(path) == false) {
    //    printf("Creating directory %s", path);
    if(mkdir(path, S_IRUSR|S_IWUSR|S_IXUSR) ==-1) {
      fprintf(stderr, "Cannot create directory %s - Error %s\n",path,strerror(errno));
      return EXIT_FAILURE;
    }
  }  
  return EXIT_SUCCESS;
}
