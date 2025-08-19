#include <stdio.h>
#include <string.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <time.h>
#include "trace.h"

#define TIME_AND_DATE_STRING_SIZE 32
#define LOG_LEVEL_STRING_SIZE 6
#define FILE_AND_LINE_SIZE 32

#define TRACE_STRING_SIZE ((TIME_AND_DATE_STRING_SIZE)+(LOG_LEVEL_STRING_SIZE)+(FILE_AND_LINE_SIZE)+(TRACE_MESSAGE_STRING_SIZE))

#define STR(s) #s

#define XSTR(s) STR(s)

typedef struct {
    bool is_initialized;
    bool rotate_logs;
    int level;
    int output;
    FILE *stream;
    size_t file_size;

}trace_internal_t;

static trace_internal_t internal_log = {0};

static char log_file[MAX_PATH_LENGTH] = "log.txt";

static const char * level_string_identifiers[] = {
    "TRACE",
    "DEBUG",
    "INFO",
    "WARN",
    "ERROR",
    "FATAL"
};

static void rotate_logs(void);





void trace_init_logger(int log_output, int level, bool rotate_logs)
{
    internal_log.level = level;
    internal_log.output = log_output;
    internal_log.is_initialized = true;
    internal_log.rotate_logs = rotate_logs;
    if ((internal_log.output & (1 << (LOG_CONSOLE - 1)))) {
        /* Initialize Console logger */
        internal_log.stream = stdout;
    }
    else {
        if ((internal_log.output & (1 << (LOG_FILE - 1)))) {
            internal_log.stream = fopen(log_file,"a");
            if (internal_log.stream == NULL) {
                fprintf(stderr, "Cannot open log file! Change to console log!");
                internal_log.stream = stdout;
                internal_log.output = LOG_CONSOLE;
                
            }
            else {
            }
        }
        else {
        }
    }
    
}

void trace_clean_logger(void)
{
    if (internal_log.output & (1 << (LOG_FILE - 1))) {
        if (internal_log.stream != NULL) {
            fclose(internal_log.stream);
        }
    }
    internal_log.is_initialized = false;
}


void trace_report_log(int level, const char *path, int line, const char* fmt,...)
{
    char time_str[TIME_AND_DATE_STRING_SIZE] = {0};
    char trace_message[TRACE_MESSAGE_STRING_SIZE] = {0};
    char trace_buffer[TRACE_STRING_SIZE] = {0};
    
    if (internal_log.is_initialized == false) {
        return;
    }
    if (internal_log.level < level) {
        return;
    }

    time_t current_time = time(NULL);
    struct tm *tm_local = localtime(&current_time);
    strftime(time_str, TIME_AND_DATE_STRING_SIZE, "%Y-%m-%d %H:%M:%S",
             tm_local);
             
    
    va_list args;
    va_start(args, fmt);
    vsnprintf(trace_message, TRACE_MESSAGE_STRING_SIZE, fmt,args);
    va_end(args);
    snprintf(trace_buffer, TRACE_STRING_SIZE,
             "%s %s %s:%d %s\n",
             time_str,
             level_string_identifiers[level],
             path,
             line,
             trace_message);
    if ((internal_log.output & (1u << (LOG_FILE - 1)))) {
        if (internal_log.rotate_logs == true) {
            if (internal_log.file_size < MAX_LOG_SIZE) {
                internal_log.file_size += (TRACE_STRING_SIZE * sizeof(char));
            }
            else {
                rotate_logs();
            }
        }
    }


    fprintf(internal_log.stream, "%s", trace_buffer);

}


void rotate_logs(void)
{

    int ret_code = 0;
    int8_t i = 0;
    char old_log_path[MAX_PATH_LENGTH] = {0};
    char new_log_path[MAX_PATH_LENGTH] = {0};
    fflush(internal_log.stream);
    fclose(internal_log.stream);

    snprintf(old_log_path, MAX_PATH_LENGTH, "%s.%d", log_file, (int8_t)(MAX_LOG_FILES));
    ret_code = remove(old_log_path);

    for (i = MAX_LOG_FILES - 1; i > 0; i--) {
        char old_path[MAX_PATH_LENGTH] = {0};
        char new_path[MAX_PATH_LENGTH] = {0};
        snprintf(old_path, MAX_PATH_LENGTH, "%s.%d", log_file, i);
        snprintf(new_path, MAX_PATH_LENGTH, "%s.%d", log_file, i + 1);
        ret_code = rename(old_path, new_path);
        
        
        
        
        
    }
    
    snprintf(new_log_path, MAX_PATH_LENGTH, "%s.1", log_file);
    rename(log_file, new_log_path);

    internal_log.stream = fopen(log_file,"a");
    if (internal_log.stream == NULL) {
        fprintf(stderr, "Cannot open log file! Change to console log!");
        internal_log.stream = stdout;
        internal_log.output = LOG_CONSOLE;
        
    }
    else {
    }
        
    
}
