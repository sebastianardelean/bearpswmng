#ifndef TRACE_H
#define TRACE_H

typedef enum {
    LVL_LOG_TRACE,
    LVL_LOG_DEBUG,
    LVL_LOG_INFO,
    LVL_LOG_WARN,
    LVL_LOG_ERROR,
    LVL_LOG_FATAL
}trace_lvl_t;


typedef enum {
    LOG_FILE = 1,
    LOG_CONSOLE = 2
}trace_output_t;

#define TRACE_MESSAGE_STRING_SIZE 512

#define MAX_LOG_SIZE 10//(1024 * 1024)  // 1MB
#define MAX_LOG_FILES 10 
#define MAX_PATH_LENGTH 256

#define LOG_TRACE(...) trace_report_log(LVL_LOG_TRACE, __FILE__, __LINE__, __VA_ARGS__)
#define LOG_DEBUG(...) trace_report_log(LVL_LOG_DEBUG, __FILE__, __LINE__, __VA_ARGS__)
#define LOG_INFO(...)  trace_report_log(LVL_LOG_INFO,  __FILE__, __LINE__, __VA_ARGS__)
#define LOG_WARN(...)  trace_report_log(LVL_LOG_WARN,  __FILE__, __LINE__, __VA_ARGS__)
#define LOG_ERROR(...) trace_report_log(LVL_LOG_ERROR, __FILE__, __LINE__, __VA_ARGS__)
#define LOG_FATAL(...) trace_report_log(LVL_LOG_FATAL, __FILE__, __LINE__, __VA_ARGS__)


void trace_report_log(int level, const char *path, int line, const char* fmt,...);

void trace_init_logger(int log_output, int level, bool rotate_logs);

void trace_clean_logger(void);
                       

#endif
